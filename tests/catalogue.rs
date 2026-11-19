use gpui::{IntoElement, Render};

fn assert_native<T: IntoElement>() {}
fn assert_native_view<T: Render>() {}

/// Source inventory and native export availability must stay in sync.
#[test]
fn original_catalogue_families_have_native_elements() {
    assert_native::<astryx_gpui::AppShell>();
    assert_native::<astryx_gpui::AspectRatio>();
    assert_native::<astryx_gpui::Avatar>();
    assert_native::<astryx_gpui::AvatarGroup>();
    assert_native::<astryx_gpui::Badge>();
    assert_native::<astryx_gpui::Banner>();
    assert_native::<astryx_gpui::Blockquote>();
    assert_native::<astryx_gpui::BottomSheet>();
    assert_native::<astryx_gpui::Breadcrumbs>();
    assert_native::<astryx_gpui::Button>();
    assert_native::<astryx_gpui::ButtonGroup>();
    assert_native::<astryx_gpui::IconButton>();
    assert_native::<astryx_gpui::Card>();
    assert_native::<astryx_gpui::ClickableCard>();
    assert_native::<astryx_gpui::Carousel>();
    assert_native::<astryx_gpui::SnapPaging>();
    assert_native::<astryx_gpui::Calendar>();
    assert_native::<astryx_gpui::Center>();
    assert_native::<astryx_gpui::CodeBlock>();
    assert_native::<astryx_gpui::CommandPalette>();
    assert_native::<astryx_gpui::ComplexSelector>();
    assert_native::<astryx_gpui::Chat>();
    assert_native::<astryx_gpui::Markdown>();
    assert_native::<astryx_gpui::Citation>();
    assert_native::<astryx_gpui::Checkbox>();
    assert_native::<astryx_gpui::CheckboxList>();
    assert_native::<astryx_gpui::Collapsible>();
    assert_native::<astryx_gpui::RadioList>();
    assert_native::<astryx_gpui::Resizable>();
    assert_native::<astryx_gpui::ScrollableArea>();
    assert_native::<astryx_gpui::Divider>();
    assert_native::<astryx_gpui::VisuallyHidden>();
    assert_native::<astryx_gpui::EmptyState>();
    assert_native::<astryx_gpui::Lightbox>();
    assert_native::<astryx_gpui::Link>();
    assert_native::<astryx_gpui::List>();
    assert_native::<astryx_gpui::MetadataList>();
    assert_native::<astryx_gpui::NavIcon>();
    assert_native::<astryx_gpui::NavMenu>();
    assert_native::<astryx_gpui::Slider>();
    assert_native::<astryx_gpui::Stack>();
    assert_native::<astryx_gpui::Stepper>();
    assert_native::<astryx_gpui::Switch>();
    assert_native::<astryx_gpui::DateInput>();
    assert_native::<astryx_gpui::DateTimeInput>();
    assert_native::<astryx_gpui::DateRangeInput>();
    assert_native::<astryx_gpui::Field>();
    assert_native::<astryx_gpui::FileInput>();
    assert_native::<astryx_gpui::FormLayout>();
    assert_native::<astryx_gpui::Grid>();
    assert_native::<astryx_gpui::Section>();
    assert_native::<astryx_gpui::SegmentedControl>();
    assert_native::<astryx_gpui::SelectableCard>();
    assert_native::<astryx_gpui::Select>();
    assert_native::<astryx_gpui::MultiSelector>();
    assert_native::<astryx_gpui::Icon>();
    assert_native::<astryx_gpui::InfoTip>();
    assert_native::<astryx_gpui::Indicator>();
    assert_native::<astryx_gpui::InputGroup>();
    assert_native::<astryx_gpui::Item>();
    assert_native::<astryx_gpui::Swipe>();
    assert_native::<astryx_gpui::Text>();
    assert_native::<astryx_gpui::Input>();
    assert_native::<astryx_gpui::Tabs>();
    assert_native::<astryx_gpui::TextArea>();
    assert_native::<astryx_gpui::TimeInput>();
    assert_native::<astryx_gpui::NumberInput>();
    assert_native::<astryx_gpui::Table>();
    assert_native::<astryx_gpui::ToggleButton>();
    assert_native::<astryx_gpui::Token>();
    assert_native::<astryx_gpui::Typeahead>();
    assert_native::<astryx_gpui::Tokenizer>();
    assert_native::<astryx_gpui::Thumbnail>();
    assert_native::<astryx_gpui::PowerSearch>();
    assert_native::<astryx_gpui::TreeList>();
    assert_native::<astryx_gpui::Kbd>();
    assert_native::<astryx_gpui::AlertDialog>();
    assert_native::<astryx_gpui::Dialog>();
    assert_native::<astryx_gpui::ContextMenu>();
    assert_native::<astryx_gpui::DropdownMenu>();
    assert_native::<astryx_gpui::Drawer>();
    assert_native::<astryx_gpui::MoreMenu>();
    assert_native::<astryx_gpui::Toolbar>();
    assert_native::<astryx_gpui::TopNav>();
    assert_native::<astryx_gpui::SideNav>();
    assert_native::<astryx_gpui::MobileNav>();
    assert_native::<astryx_gpui::Pagination>();
    assert_native::<astryx_gpui::ProgressBar>();
    assert_native::<astryx_gpui::Layout>();
    assert_native::<astryx_gpui::Layer>();
    assert_native::<astryx_gpui::Toast>();
    assert_native::<astryx_gpui::Popover>();
    assert_native::<astryx_gpui::HoverCard>();
    // Tooltip is mounted as an Entity through the host's native tooltip lifecycle.
    assert_native_view::<astryx_gpui::Tooltip>();
    assert_native::<astryx_gpui::Skeleton>();
    assert_native::<astryx_gpui::StatusDot>();
    assert_native::<astryx_gpui::Spinner>();
    assert_native::<astryx_gpui::Timestamp>();
    assert_native::<astryx_gpui::Timer>();
    assert_native::<astryx_gpui::Overlay>();
    assert_native::<astryx_gpui::Outline>();
    assert_native::<astryx_gpui::OverflowList>();
}

#[test]
fn coverage_records_every_pinned_source_ui_family_once() {
    use std::collections::BTreeSet;
    let expected: BTreeSet<_> = include_str!("../docs/upstream-ui-families.txt")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect();
    let rows: Vec<_> = include_str!("../docs/coverage.md")
        .lines()
        .filter_map(|line| {
            line.strip_prefix("| [")
                .and_then(|value| value.split_once(']'))
                .map(|(name, _)| name)
        })
        .collect();
    assert_eq!(expected.len(), 102);
    assert_eq!(rows.len(), expected.len());
    assert_eq!(rows.into_iter().collect::<BTreeSet<_>>(), expected);
}
