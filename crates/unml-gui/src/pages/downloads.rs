mod completed;
mod failed;
mod in_progress;

use completed::CompletedPage;
use failed::FailedPage;
use in_progress::InProgressPage;
use unml_macros::PageRoute;

#[derive(PageRoute)]
#[route(id = "downloads", label = "nav.downloads", icon = ArrowDown)]
#[sidebar(
    variant = Filter,
    section {
        InProgress => "downloads.in_progress",
        Completed => "downloads.completed",
        Failed => "downloads.failed",
    }
)]
#[children(InProgressPage, CompletedPage, FailedPage)]
pub struct DownloadsPage;
