pub mod progresse_bar_custom {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::time::Duration;

    pub struct ProgressBarCustom {
        bar: ProgressBar,
        launch: bool,
    }

    /// Handle all action about the progressbar
    impl ProgressBarCustom {
        /// create the progressbar
        pub fn create() -> ProgressBarCustom {
            let pb = indicatif::ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::with_template("{spinner:.blue} {msg}")
                    .unwrap()
                    .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
            );
            ProgressBarCustom {
                bar: pb,
                launch: false,
            }
        }

        /// launch the progressbar
        pub fn launch(&mut self) {
            self.bar.set_message("Calculating...");
            self.bar.enable_steady_tick(Duration::from_millis(80));
            self.launch = true;
        }

        /// End the progressbar
        pub fn done(&self) {
            if self.launch {
                self.bar.finish_with_message("Done ✅");
            }
        }
    }
}
