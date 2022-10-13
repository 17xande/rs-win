use windows::{
    core::*, 
    Win32::UI::Shell::*,
};

#[implement(IDesktopWallpaper)]
struct Dw();

impl IDesktopWallpaper_Impl for Dw {
    fn GetWallpaper(&self,monitorid: &windows::core::PCWSTR) ->  windows::core::Result<windows::core::PWSTR> {
        todo!("Your implementation goes here");
    }

    fn GetMonitorDevicePathAt(&self,monitorindex:u32) ->  windows::core::Result<windows::core::PWSTR> {
        todo!("Your implementation goes here");
    }

    fn GetMonitorDevicePathCount(&self) ->  windows::core::Result<u32> {
        todo!("Your implementation goes here");
    }

    fn GetMonitorRECT(&self,monitorid: &windows::core::PCWSTR) ->  windows::core::Result<windows::Win32::Foundation::RECT> {
        todo!("Your implementation goes here");
    }

    fn GetBackgroundColor(&self) ->  windows::core::Result<windows::Win32::Foundation::COLORREF> {
        todo!("Your implementation goes here");
    }

    fn SetBackgroundColor(&self,color:windows::Win32::Foundation::COLORREF) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn SetPosition(&self,position:DESKTOP_WALLPAPER_POSITION) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn GetPosition(&self) ->  windows::core::Result<DESKTOP_WALLPAPER_POSITION> {
        todo!("Your implementation goes here");
    }

    fn GetSlideshow(&self) ->  windows::core::Result<IShellItemArray> {
        todo!("Your implementation goes here");
    }

    fn SetSlideshow(&self,items: &core::option::Option<IShellItemArray>) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn SetWallpaper(&self,monitorid: &windows::core::PCWSTR,wallpaper: &windows::core::PCWSTR) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn SetSlideshowOptions(&self,options:DESKTOP_SLIDESHOW_OPTIONS,slideshowtick:u32) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn GetSlideshowOptions(&self,options: *mut DESKTOP_SLIDESHOW_OPTIONS,slideshowtick: *mut u32) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn AdvanceSlideshow(&self,monitorid: &windows::core::PCWSTR,direction:DESKTOP_SLIDESHOW_DIRECTION) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

    fn GetStatus(&self) ->  windows::core::Result<DESKTOP_SLIDESHOW_STATE> {
        todo!("Your implementation goes here");
    }

    fn Enable(&self,enable:windows::Win32::Foundation::BOOL) ->  windows::core::Result<()> {
        todo!("Your implementation goes here");
    }

}

fn main() -> Result<()> {
    let name = w!("wallpaper1.jpg");
    let wallpaper: IDesktopWallpaper = Dw().into();
    unsafe {
        wallpaper.SetWallpaper(w!("0"), name);
    }

    Ok(())
}
