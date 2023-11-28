
pub struct ExtlinuxEntry {
    pub label: Option<String>,
    pub menu_label: Option<String>,
    pub linux: Option<String>,
    pub fdt: Option<String>,
    pub initrd: Option<String>,
    pub append: Option<String>,
}

impl ExtlinuxEntry {
    pub fn init(&mut self, content: &[&str]) {
        for line in content {
            if line.starts_with("LABEL") {
                let label_string = line.strip_prefix("LABEL").unwrap().trim().to_string();
                self.label = Some(label_string);
            }

            if line.trim().starts_with("MENU LABEL") {
                let menu_label_string = line.trim().strip_prefix("MENU LABEL").unwrap().trim().to_string();
                self.menu_label = Some(menu_label_string);
            }

            if line.trim().starts_with("LINUX") {
                let linux_string = line.trim().strip_prefix("LINUX").unwrap().trim().to_string();
                self.linux = Some(linux_string);
            }

            if line.trim().starts_with("FDT") {
                let fdt_string = line.trim().strip_prefix("FDT").unwrap().trim().to_string();
                self.fdt = Some(fdt_string);
            }

            if line.trim().starts_with("INITRD") {
                let initrd_string = line.trim().strip_prefix("INITRD").unwrap().trim().to_string();
                self.initrd = Some(initrd_string);
            }

            if line.trim().starts_with("APPEND") {
                let append_string = line.trim().strip_prefix("APPEND").unwrap().trim().to_string();
                self.append = Some(append_string);
            }
        }
    }
}
