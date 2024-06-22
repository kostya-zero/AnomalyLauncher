fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "Anomaly Launcher");
    res.set("FileDescription", "Anomaly Launcher");
    res.set("LegalCopyright", "Copyright (C) 2024");
    res.set_icon("assets/icon.ico");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");
}