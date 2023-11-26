use windows::core::ComInterface;
use windows::Win32::{
    Foundation::HWND, Graphics::Direct3D::*, Graphics::Direct3D12::*, Graphics::Dxgi::Common::*,
    Graphics::Dxgi::*, System::Com::*,
};

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED | COINIT_DISABLE_OLE1DDE).unwrap();
    }
    unsafe {
        let debug = {
            let mut p: Option<ID3D12Debug> = None;
            D3D12GetDebugInterface(&mut p).map(|_| p.unwrap())
        };
        if let Ok(debug) = debug {
            debug.EnableDebugLayer();
            println!("enabled d3d12 debug layer");
        }
    }
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("pnte d3d12")
        .build()?;
    let size = window.inner_size().unwrap();
    let device: ID3D12Device = unsafe {
        let mut p = None;
        D3D12CreateDevice(None, D3D_FEATURE_LEVEL_12_0, &mut p).map(|_| p.unwrap())?
    };
    let cmd_queue: ID3D12CommandQueue = unsafe {
        device.CreateCommandQueue(&D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            ..Default::default()
        })?
    };
    let cmd_allocator: ID3D12CommandAllocator =
        unsafe { device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)? };
    let cmd_list: ID3D12GraphicsCommandList = unsafe {
        device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &cmd_allocator, None)?
    };
    unsafe {
        cmd_list.Close()?;
    }
    let dxgi_factory: IDXGIFactory6 = unsafe { CreateDXGIFactory1()? };
    let swap_chain: IDXGISwapChain4 = unsafe {
        dxgi_factory
            .CreateSwapChainForHwnd(
                &cmd_queue,
                HWND(window.raw_handle()),
                &DXGI_SWAP_CHAIN_DESC1 {
                    Width: size.width,
                    Height: size.height,
                    Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    BufferCount: 2,
                    BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    ..Default::default()
                },
                None,
                None,
            )?
            .cast()?
    };
    let rtv_heap: ID3D12DescriptorHeap = unsafe {
        device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: 2,
            ..Default::default()
        })?
    };
    let inc = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) };
    let buffers = unsafe {
        (0..2)
            .map(|i| -> anyhow::Result<ID3D12Resource> {
                let mut handle = rtv_heap.GetCPUDescriptorHandleForHeapStart();
                handle.ptr += inc as usize * i;
                let buffer: ID3D12Resource = swap_chain.GetBuffer(i as u32)?;
                device.CreateRenderTargetView(&buffer, None, handle);
                Ok(buffer)
            })
            .collect::<anyhow::Result<Vec<ID3D12Resource>>>()?
    };
    let ctx = pnte::Context::new(pnte::Direct3D12::new(&device, &cmd_queue, 0)?)?;
    let targets = buffers
        .iter()
        .map(|buffer| -> anyhow::Result<pnte::d3d12::RenderTarget> {
            let target = ctx.create_render_target(buffer)?;
            Ok(target)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let fence: ID3D12Fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE)? };
    let mut next_frame = 1u64;
    loop {
        match event_rx.try_recv() {
            Ok((event, _)) => match event {
                _ => {}
            },
            Err(wiard::TryRecvError::Empty) => unsafe {
                let index = swap_chain.GetCurrentBackBufferIndex() as usize;
                let mut handle = rtv_heap.GetCPUDescriptorHandleForHeapStart();
                handle.ptr += inc as usize * index;
                let buffer = &buffers[index];
                let target = &targets[index];
                cmd_allocator.Reset()?;
                cmd_list.Reset(&cmd_allocator, None)?;
                let barriers = [D3D12_RESOURCE_BARRIER {
                    Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                    Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                    Anonymous: D3D12_RESOURCE_BARRIER_0 {
                        Transition: std::mem::ManuallyDrop::new(
                            D3D12_RESOURCE_TRANSITION_BARRIER {
                                pResource: std::mem::ManuallyDrop::new(Some(buffer.clone())),
                                Subresource: 0,
                                StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                                StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
                            },
                        ),
                    },
                }];
                cmd_list.ResourceBarrier(&barriers);
                barriers.into_iter().for_each(|barrier| {
                    let t = std::mem::ManuallyDrop::into_inner(barrier.Anonymous.Transition);
                    std::mem::ManuallyDrop::into_inner(t.pResource);
                });
                cmd_list.ClearRenderTargetView(handle, &[0.0, 0.0, 0.3, 0.0], None);
                let barriers = [D3D12_RESOURCE_BARRIER {
                    Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                    Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                    Anonymous: D3D12_RESOURCE_BARRIER_0 {
                        Transition: std::mem::ManuallyDrop::new(
                            D3D12_RESOURCE_TRANSITION_BARRIER {
                                pResource: std::mem::ManuallyDrop::new(Some(buffer.clone())),
                                Subresource: 0,
                                StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
                                StateAfter: D3D12_RESOURCE_STATE_PRESENT,
                            },
                        ),
                    },
                }];
                cmd_list.ResourceBarrier(&barriers);
                barriers.into_iter().for_each(|barrier| {
                    let t = std::mem::ManuallyDrop::into_inner(barrier.Anonymous.Transition);
                    std::mem::ManuallyDrop::into_inner(t.pResource);
                });
                cmd_list.Close()?;
                cmd_queue.ExecuteCommandLists(&[Some(cmd_list.cast().unwrap())]);
                ctx.draw(target, |cmd| {
                    let color = pnte::SolidColorBrush::new(&ctx, (0.5, 0.8, 0.0, 1.0)).unwrap();
                    cmd.fill(
                        &pnte::Rect::from_point_size((50.0, 50.0), (100.0, 100.0)),
                        &color,
                    );
                })?;
                swap_chain.Present(0, 0).ok()?;
                let frame = next_frame;
                next_frame += 1;
                cmd_queue.Signal(&fence, frame)?;
                if fence.GetCompletedValue() < frame {
                    fence.SetEventOnCompletion(frame, None)?;
                }
            },
            Err(wiard::TryRecvError::Disconnected) => break,
        }
    }
    Ok(())
}
