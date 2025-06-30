use pacmanconf::Config;

pub trait Export {
    fn export(&self) -> String;
}

impl Export for Config {
    fn export(&self) -> String {
        let mut result = String::with_capacity(1024*4);

        result.push_str("[options]\n");
    
        match self.root_dir.as_str() {
            "" => result.push_str("#RootDir = \n"),
            dir => result.push_str(&format!("RootDir = {}\n", dir)),
        }

        match self.db_path.as_str() {
            "" => result.push_str("#DBPath = \n"),
            dir => result.push_str(&format!("DBPath = {}\n", dir)),
        }

        match !self.cache_dir.is_empty() {
            false => result.push_str("#CacheDir = \n"),
            true => result.push_str(&format!("CacheDir = {}\n", self.cache_dir.join(" "))),
        }

        match self.log_file.is_empty() {
            true => result.push_str("#LogFile = \n"),
            false => result.push_str(&format!("LogFile = {}\n", self.log_file)),
        }

        match self.gpg_dir.is_empty(){
            true => result.push_str("#GPGDir = \n"),
            false => result.push_str(&format!("GPGDir = {}\n", self.gpg_dir)),
        }

        match self.hook_dir.is_empty() {
            true => result.push_str("#HookDir = \n"),
            false => result.push_str(&format!("HookDir = {}\n", self.hook_dir.join(" "))),
        }

        match self.hold_pkg.is_empty() {
            true => result.push_str("#HoldPkg = \n"),
            false => result.push_str(&format!("HoldPkg = {}\n", self.hold_pkg.join(" "))),
        }

        match self.xfer_command.is_empty() {
            true => result.push_str("#XferCommand = \n"),
            false => result.push_str(&format!("XferCommand = {}\n", self.xfer_command)),
        }

        match self.clean_method.is_empty(){
            true => result.push_str("#CleanMethod = \n"),
            false => result.push_str(&format!("CleanMethod = {}\n", self.clean_method.join(" "))),
        }

        match self.architecture.is_empty(){
            true => result.push_str("#Architecture = \n"),
            false => result.push_str(&format!("Architecture = {}\n", self.architecture.join(" "))),
        }

        result.push('\n');

        match self.ignore_pkg.is_empty(){
            true => result.push_str("#IgnorePkg = \n"),
            false => result.push_str(&format!("IgnorePkg = {}\n", self.ignore_pkg.join(" "))),
        }

        match self.ignore_group.is_empty(){
            true => result.push_str("#IgnoreGroup = \n"),
            false => result.push_str(&format!("IgnoreGroup = {}\n", self.ignore_group.join(" "))),
        }

        result.push('\n');

        match self.no_upgrade.is_empty(){
            true => result.push_str("#NoUpgrade = \n"),
            false => result.push_str(&format!("NoUpgrade = {}\n", self.no_upgrade.join(" "))),
        }

        match self.no_extract.is_empty(){
            true => result.push_str("#NoExtract = \n"),
            false => result.push_str(&format!("NoExtract = {}\n", self.no_extract.join(" "))),
        }

        result.push('\n');

        match self.use_syslog {
            false => result.push_str("#UseSyslog\n"),
            true => result.push_str("UseSyslog\n"),
        }

        match self.color {
            true => result.push_str("Color\n"),
            false => result.push_str("#Color\n"),
        }

        match self.check_space {
            true => result.push_str("CheckSpace\n"),
            false => result.push_str("#CheckSpace\n"),
        }

        match self.verbose_pkg_lists {
            true => result.push_str("VerbosePkgLists\n"),
            false => result.push_str("#VerbosePkgLists\n"),
        }

        result.push_str(&format!("ParallelDownloads = {}\n", self.parallel_downloads));

        match self.download_user.as_ref() {
            Some(user) => result.push_str(&format!("DownloadUser = {}\n", user)),
            None => result.push_str("#DownloadUser = \n"),
        }

        match self.disable_sandbox {
            true => result.push_str("DisableSandbox\n"),
            false => result.push_str("#DisableSandbox\n"),
        }

        result.push('\n');

        match self.sig_level.is_empty() {
            true => result.push_str("#SigLevel\n"),
            false => result.push_str(&format!("SigLevel = {}\n", self.sig_level.join(" "))),
        }

        match self.local_file_sig_level.is_empty() {
            true => result.push_str("#LocalFileSigLevel\n"),
            false => result.push_str(&format!("LocalFileSigLevel = {}\n", self.local_file_sig_level.join(" "))),
        }

        match self.remote_file_sig_level.is_empty() {
            true => result.push_str("#RemoteFileSigLevel\n"),
            false => result.push_str(&format!("RemoteFileSigLevel = {}\n", self.remote_file_sig_level.join(" "))),
        }


        for i in self.repos.iter(){
            result.push('\n');
            result.push_str(&format!("[{}]\n", i.name));

            for i in i.servers.iter(){
                result.push_str(&format!("Server = {}\n", i));
            }

            if !i.sig_level.is_empty(){
                result.push_str(&format!("SigLevel = {}\n", i.sig_level.join(" ")));
            }

            if !i.usage.is_empty(){
                result.push_str(&format!("Usage = {}\n", i.usage.join(" ")));
            }
        }


        result
    }
}