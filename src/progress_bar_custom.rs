pub mod progresse_bar_custom {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::time::Duration;

    pub struct ProgressBarCustom {
        bar: ProgressBar,
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
            ProgressBarCustom { bar: pb }
        }

        /// launch the progressbar
        pub fn launch(&self) {
            self.bar.set_message("Calculating...");
            self.bar.enable_steady_tick(Duration::from_millis(80));
        }

        /// End the progressbar
        pub fn done(&self) {
            self.bar.finish_with_message("Done ✅");
        }
    }
}
