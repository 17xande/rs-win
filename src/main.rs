use windows::{
    core::*, 
    Win32::UI::Shell::*,
};

#[implement(IDesktopWallpaper)]
struct Dw();

fn main() -> Result<()> {
    let name = w!("wallpaper1.jpg");
    let wallpaper: IDesktopWallpaper = Dw().into();
    unsafe {
        wallpaper.SetWallpaper(w!("0"), name);
    }

    Ok(())
}
