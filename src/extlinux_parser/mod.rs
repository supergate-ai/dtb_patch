mod entry;

use std::fs::OpenOptions;
use std::io::Read;
use entry::ExtlinuxEntry;

pub struct Extlinux {
    timeout: Option<usize>,
    default: Option<String>,
    menu_title: Option<String>,
    entries: Vec<ExtlinuxEntry>,
}

impl Extlinux {
    pub fn load(path: &str) -> Self {
        let mut ret = Self {
            timeout: None,
            default: None,
            menu_title: None,
            entries: Vec::<ExtlinuxEntry>::new(),
        };
        let mut extlinux = OpenOptions::new()
                                        .read(true)
                                        .create_new(false)
                                        .truncate(false)
                                        .write(true)
                                        .open(path)
                                        .expect("Error : Cannot open /boot/extlinux/extlinux.conf... Please run this program as superuser");
        
        let mut extlinux_content = String::new();
        extlinux.read_to_string(&mut extlinux_content).expect("Error : Cannot read from extlinux");
        let lines = extlinux_content.lines();

        for line in lines {
            if line.starts_with("TIMEOUT") {
                let timeout_string = line.strip_prefix("TIMEOUT").unwrap().trim().to_string();
                ret.timeout = Some(timeout_string.parse::<usize>().unwrap());
            }

            if line.starts_with("DEFAULT") {
                let default_string = line.strip_prefix("DEFAULT").unwrap().trim().to_string();
                ret.default = Some(default_string);
            }

            if line.starts_with("MENU TITLE") {
                let menu_title_string = line.strip_prefix("MENU TITLE").unwrap().trim().to_string();
                ret.menu_title = Some(menu_title_string);
            }
        }

        let mut label_lines = Vec::<usize>::new();
        for (idx, line) in extlinux_content.lines().enumerate() {
            if line.starts_with("LABEL") {
                label_lines.push(idx);
            }
        }
        label_lines.push(0);

        label_lines.into_iter().fold(0, |acc, x| {
            if acc > 0 {
                let mut entry = ExtlinuxEntry {
                    label: None,
                    menu_label: None,
                    linux: None,
                    fdt: None,
                    initrd: None,
                    append: None,
                };
                if x == 0 {
                    entry.init(&extlinux_content.lines().collect::<Vec::<&str>>()[acc..]);
                } else {
                    entry.init(&extlinux_content.lines().collect::<Vec::<&str>>()[acc..x]);
                }
                ret.entries.push(entry);
            }
            x
        });

        ret
    }

    pub fn default_entry(&self) -> &ExtlinuxEntry {
        &self.entries.iter().find(|&entry| entry.label == self.default).expect("Cannot find default entry")
    }
}

