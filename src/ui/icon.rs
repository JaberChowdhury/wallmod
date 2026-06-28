use iced::{widget::text, Font};

// Load the downloaded font into the binary
pub const ICON_FONT: Font = Font::with_name("bootstrap-icons");
pub const ICON_BYTES: &[u8] = include_bytes!("../../fonts/bootstrap-icons.ttf");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    Settings,
    Palette,
    Terminal,
    Folder,
    FolderOpen,
    Check,
    Image,
    ImageFill,
    Images,
    Moon,
    Sun,
    Display,
    ArrowRepeat,
    ArrowRightCircle,
    ArrowLeftCircle,
    XCircle,
    Download,
    Eye,
    EyeSlash,
    Sliders,
    Layers,
    Magic,
    List,
    PlusSquare,
    ExclamationTriangle,
    InfoCircle,
    Search,
    Grid,
    FileType,
    FileZip,
    ArrowsAngleExpand,
}

impl Icon {
    pub fn to_char(self) -> char {
        match self {
            Icon::Settings => '\u{F3E5}', 
            Icon::Palette => '\u{F498}',
            Icon::Terminal => '\u{F5BF}',
            Icon::Folder => '\u{F3D7}',
            Icon::FolderOpen => '\u{F3D8}',
            Icon::Check => '\u{F26B}',
            Icon::Image => '\u{F424}',
            Icon::ImageFill => '\u{F425}',
            Icon::Images => '\u{F426}',
            Icon::Moon => '\u{F497}',
            Icon::Sun => '\u{F5D4}',
            Icon::Display => '\u{F301}',
            Icon::ArrowRepeat => '\u{F130}',
            Icon::ArrowRightCircle => '\u{F138}',
            Icon::ArrowLeftCircle => '\u{F12F}',
            Icon::XCircle => '\u{F623}',
            Icon::Download => '\u{F30A}',
            Icon::Eye => '\u{F339}',
            Icon::EyeSlash => '\u{F338}',
            Icon::Sliders => '\u{F5B7}',
            Icon::Layers => '\u{F452}',
            Icon::Magic => '\u{F47F}',
            Icon::List => '\u{F479}',
            Icon::PlusSquare => '\u{F4E5}',
            Icon::ExclamationTriangle => '\u{F33B}',
            Icon::InfoCircle => '\u{F431}',
            Icon::Search => '\u{F52A}',
            Icon::Grid => '\u{F3E2}',
            Icon::FileType => '\u{F398}', // file-text
            Icon::FileZip => '\u{F3A0}',
            Icon::ArrowsAngleExpand => '\u{F124}',
        }
    }
}

/// The global helper function to render an icon in the Iced UI
pub fn icon<'a>(icon: Icon) -> iced::widget::Text<'a> {
    text(icon.to_char().to_string())
        .font(ICON_FONT)
}
