use super::*;
use windows::Win32::{
    Foundation::{HMODULE, HWND},
    Graphics::Direct3D::*,
    Graphics::Direct3D11::*,
    Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC},
    Graphics::Dxgi::*,
};

#[derive(Clone)]
pub struct RenderTarget {
    ctx: Context<Direct2D>,
    swap_chain: IDXGISwapChain1,
    render_target: Option<d3d11::RenderTarget>,
    interval: u32,
}

impl RenderTarget {
    #[inline]
    pub fn set_interval(&mut self, interval: u32) {
        self.interval = interval;
    }

    #[inline]
    pub fn resize(&mut self, size: impl Into<Size<u32>>) -> Result<()> {
        let size: Size<u32> = size.into();
        self.render_target = None;
        unsafe {
            self.swap_chain.ResizeBuffers(
                0,
                size.width,
                size.height,
                DXGI_FORMAT_UNKNOWN,
                DXGI_SWAP_CHAIN_FLAG(0),
            )?;
        }
        self.render_target = Some(
            self.ctx
                .backend
                .d3d11
                .create_render_target_from_swap_chain(
                    &self.ctx.d2d1_device_context,
                    &self.swap_chain,
                )?,
        );
        Ok(())
    }
}

impl Target for RenderTarget {
    fn bitmap(&self) -> &ID2D1Bitmap1 {
        self.render_target.as_ref().unwrap().bitmap()
    }

    fn size(&self) -> Size<f32> {
        self.render_target.as_ref().unwrap().size()
    }

    fn pixel_size(&self) -> Size<u32> {
        self.render_target.as_ref().unwrap().pixel_size()
    }
}

impl PartialEq for RenderTarget {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.swap_chain == other.swap_chain
    }
}

impl Eq for RenderTarget {}

#[derive(Clone)]
pub struct Direct2D {
    d3d11_device: ID3D11Device,
    d3d11: Direct3D11,
    dxgi_factory: IDXGIFactory2,
}

impl Direct2D {
    pub fn new() -> Result<Self> {
        unsafe {
            let d3d11_device: ID3D11Device = {
                let mut p = None;
                D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    HMODULE::default(),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    Some(&[D3D_FEATURE_LEVEL_11_0]),
                    D3D11_SDK_VERSION,
                    Some(&mut p),
                    None,
                    None,
                )
                .map(|_| p.unwrap())?
            };
            let dxgi_factory: IDXGIFactory2 = CreateDXGIFactory1()?;
            let d3d11 = Direct3D11::new(&d3d11_device)?;
            Ok(Self {
                d3d11_device,
                dxgi_factory,
                d3d11,
            })
        }
    }
}

impl Context<Direct2D> {
    pub fn create_render_target(
        &self,
        window: impl raw_window_handle::HasWindowHandle,
        size: impl Into<Size<u32>>,
    ) -> Result<RenderTarget> {
        let size: Size<u32> = size.into();
        let raw_window_handle::RawWindowHandle::Win32(window) =
            window.window_handle().unwrap().as_raw()
        else {
            panic!("no support the window handle.");
        };
        let hwnd = HWND(isize::from(window.hwnd) as *mut std::ffi::c_void);
        unsafe {
            let swap_chain = self.backend.dxgi_factory.CreateSwapChainForHwnd(
                &self.backend.d3d11_device,
                hwnd,
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
            )?;
            let render_target = self
                .backend
                .d3d11
                .create_render_target_from_swap_chain(&self.d2d1_device_context, &swap_chain)?;
            Ok(RenderTarget {
                ctx: self.clone(),
                swap_chain,
                render_target: Some(render_target),
                interval: 1,
            })
        }
    }
}

impl Backend for Direct2D {
    type RenderTarget = RenderTarget;

    fn d2d1_device(&self) -> &ID2D1Device5 {
        self.d3d11.d2d1_device()
    }

    fn d2d1_factory(&self) -> &ID2D1Factory6 {
        self.d3d11.d2d1_factory()
    }

    fn begin_draw(&self, target: &Self::RenderTarget) {
        self.d3d11
            .begin_draw(target.render_target.as_ref().unwrap());
    }

    fn end_draw(&self, target: &Self::RenderTarget) -> Result<()> {
        self.d3d11
            .end_draw(target.render_target.as_ref().unwrap())?;
        unsafe {
            let params = DXGI_PRESENT_PARAMETERS::default();
            target
                .swap_chain
                .Present1(target.interval, DXGI_PRESENT(0), &params)
                .ok()?;
        }
        Ok(())
    }
}
