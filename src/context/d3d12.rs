use super::*;
use windows::Win32::{
    Foundation::E_FAIL, Graphics::Direct2D::Common::*, Graphics::Direct2D::*,
    Graphics::Direct3D11::*, Graphics::Direct3D11on12::*, Graphics::Direct3D12::*,
    Graphics::Dxgi::*,
};
use windows::core::{IUnknown, Interface};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenderTarget {
    wrapper: ID3D11Resource,
    bitmap: ID2D1Bitmap1,
}

impl Target for RenderTarget {
    #[inline]
    fn bitmap(&self) -> &ID2D1Bitmap1 {
        &self.bitmap
    }

    #[inline]
    fn size(&self) -> Size<f32> {
        unsafe { self.bitmap.GetSize().into() }
    }

    #[inline]
    fn pixel_size(&self) -> Size<u32> {
        unsafe { self.bitmap.GetPixelSize().into() }
    }
}

#[derive(Clone, Debug)]
pub struct Direct3D12 {
    d3d11on12_device: ID3D11On12Device,
    d2d1_factory: ID2D1Factory6,
    d2d1_device: ID2D1Device5,
    d3d11_device_context: ID3D11DeviceContext,
}

impl Direct3D12 {
    pub fn new<T, U>(d3d12_device: &T, command_queue: &U, nodemask: u32) -> Result<Self>
    where
        T: Interface,
        U: Interface,
    {
        let d3d12_device: ID3D12Device = d3d12_device.cast()?;
        let command_queue: ID3D12CommandQueue = command_queue.cast()?;
        let (d3d11on12_device, d3d11_device_context) = unsafe {
            let queues = [Some(command_queue.cast::<IUnknown>()?)];
            let mut device = None;
            let mut dc = None;
            D3D11On12CreateDevice(
                &d3d12_device,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT.0,
                None,
                Some(&queues),
                nodemask,
                Some(&mut device),
                Some(&mut dc),
                None,
            )
            .map(|_| {
                (
                    device.unwrap().cast::<ID3D11On12Device>().unwrap(),
                    dc.unwrap(),
                )
            })?
        };
        let d2d1_factory: ID2D1Factory6 =
            unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, None)? };
        let dxgi_device: IDXGIDevice = d3d11on12_device.cast()?;
        let d2d1_device = unsafe { d2d1_factory.CreateDevice(&dxgi_device)? };
        Ok(Self {
            d3d11on12_device,
            d2d1_factory,
            d2d1_device,
            d3d11_device_context,
        })
    }
}

impl Context<Direct3D12> {
    pub fn create_render_target<T>(&self, target: &T) -> Result<RenderTarget>
    where
        T: Interface,
    {
        unsafe {
            let resource: ID3D12Resource = target.cast()?;
            let desc = resource.GetDesc();
            if desc.Flags & D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET == D3D12_RESOURCE_FLAG_NONE {
                return Err(windows::core::Error::from(E_FAIL).into());
            }
            let wrapper = {
                let mut wrapper: Option<ID3D11Resource> = None;
                self.backend
                    .d3d11on12_device
                    .CreateWrappedResource(
                        &resource,
                        &D3D11_RESOURCE_FLAGS {
                            BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
                            ..Default::default()
                        },
                        D3D12_RESOURCE_STATE_RENDER_TARGET,
                        D3D12_RESOURCE_STATE_COMMON,
                        &mut wrapper,
                    )
                    .map(|_| wrapper.unwrap())?
            };
            let surface: IDXGISurface = wrapper.cast()?;
            let bitmap = self.d2d1_device_context.CreateBitmapFromDxgiSurface(
                &surface,
                Some(&D2D1_BITMAP_PROPERTIES1 {
                    pixelFormat: D2D1_PIXEL_FORMAT {
                        format: desc.Format,
                        alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                    },
                    bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                    ..Default::default()
                }),
            )?;
            Ok(RenderTarget { wrapper, bitmap })
        }
    }

    #[inline]
    pub fn flush(&self) {
        unsafe {
            self.backend.d3d11_device_context.Flush();
        }
    }
}

impl Backend for Direct3D12 {
    type RenderTarget = RenderTarget;

    fn d2d1_device(&self) -> &ID2D1Device5 {
        &self.d2d1_device
    }

    fn d2d1_factory(&self) -> &ID2D1Factory6 {
        &self.d2d1_factory
    }

    fn begin_draw(&self, target: &Self::RenderTarget) {
        unsafe {
            self.d3d11on12_device
                .AcquireWrappedResources(&[Some(target.wrapper.clone())]);
        }
    }

    fn end_draw(&self, target: &Self::RenderTarget) -> Result<()> {
        unsafe {
            self.d3d11on12_device
                .ReleaseWrappedResources(&[Some(target.wrapper.clone())]);
            self.d3d11_device_context.Flush();
            self.d3d11_device_context.ClearState();
        }
        Ok(())
    }
}
