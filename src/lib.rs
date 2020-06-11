use zathura_plugin::*;
use std::fs;
use pulldown_cmark::{Parser, Options};
use pango::LayoutExt;
use pango::FontDescription;

struct PluginType {
    markdown: String,
}

impl ZathuraPlugin for PluginType {
    type DocumentData = String;
    type PageData = ();

    fn document_open(doc: DocumentRef<'_>) -> Result<DocumentInfo<Self>, PluginError> {
        let path = doc.path_utf8();
        let markdown = match path {
            Ok(path) => fs::read_to_string(path).unwrap(),
            Err(_) => "".to_string(),
        };

        let doc = DocumentInfo {
            page_count: 1,
            plugin_data: markdown,
        };
        Ok(doc)
    }

    fn page_init(
        page: PageRef<'_>,
        doc_data: &mut Self::DocumentData,
    ) -> Result<PageInfo<Self>, PluginError> {
        let page = PageInfo {
            width: 600.0,
            height: 900.0,
            plugin_data: (),
        };
        Ok(page)
    }

    fn page_render(
        page: PageRef<'_>,
        doc_data: &mut Self::DocumentData,
        page_data: &mut Self::PageData,
        cairo: &mut cairo::Context,
        printing: bool,
    ) -> Result<(), PluginError> {
        let line_height = 25.0;
        let indent = 5.0;
        let context = pangocairo::functions::create_context(cairo).expect("Creating pango context failed!");
        let mut line = 0.0;
        let parser = Parser::new_ext(doc_data, Options::empty());
        let mut last_size: Option<i32> = None;
        let mut last_family: Option<&str> = None;
        let mut last_style: Option<pango::Style> = None;
        let mut last_weight: Option<pango::Weight> = None;
        for event in parser {
            let layout = pangocairo::functions::create_layout(cairo).unwrap();
            let mut font_desc = FontDescription::new();
            match event {
                pulldown_cmark::Event::Start(t) => {
                    match t {
                        pulldown_cmark::Tag::Paragraph => {}
                        pulldown_cmark::Tag::Heading(h) => {
                            last_weight = Some(pango::Weight::Bold);
                            match h {
                                1 => {
                                    last_size = Some(20);
                                }
                                2 => {
                                    last_size = Some(18);
                                }
                                3 => {
                                    last_size = Some(16);
                                }
                                4 => {
                                    last_size = Some(14);
                                }
                                5 => {
                                    last_size = Some(12);
                                }
                                _ => {}
                            }
                        }
                        pulldown_cmark::Tag::BlockQuote => {}
                        pulldown_cmark::Tag::CodeBlock(t) => {
                            last_family = Some("mono");
                            match t {
                                pulldown_cmark::CodeBlockKind::Indented => {}
                                pulldown_cmark::CodeBlockKind::Fenced(_) => {}
                            }
                        }
                        pulldown_cmark::Tag::List(_) => {}
                        pulldown_cmark::Tag::Item => {}
                        pulldown_cmark::Tag::FootnoteDefinition(_) => {}
                        pulldown_cmark::Tag::Table(_) => {}
                        pulldown_cmark::Tag::TableHead => {}
                        pulldown_cmark::Tag::TableRow => {}
                        pulldown_cmark::Tag::TableCell => {}
                        pulldown_cmark::Tag::Emphasis => {
                            last_style = Some(pango::Style::Italic);
                        }
                        pulldown_cmark::Tag::Strong => {
                            last_weight = Some(pango::Weight::Bold);
                        }
                        pulldown_cmark::Tag::Strikethrough => {}
                        pulldown_cmark::Tag::Link(_, _, _) => {}
                        pulldown_cmark::Tag::Image(_, _, _) => {}
                    }
                }
                pulldown_cmark::Event::End(t) => {
                    match t {
                        pulldown_cmark::Tag::Paragraph => {}
                        pulldown_cmark::Tag::Heading(_) => {
                            last_size = None;
                            last_style = None;
                            last_weight = None;
                        }
                        pulldown_cmark::Tag::BlockQuote => {}
                        pulldown_cmark::Tag::CodeBlock(_) => {
                            last_family = None;
                        }
                        pulldown_cmark::Tag::List(_) => {}
                        pulldown_cmark::Tag::Item => {}
                        pulldown_cmark::Tag::FootnoteDefinition(_) => {}
                        pulldown_cmark::Tag::Table(_) => {}
                        pulldown_cmark::Tag::TableHead => {}
                        pulldown_cmark::Tag::TableRow => {}
                        pulldown_cmark::Tag::TableCell => {}
                        pulldown_cmark::Tag::Emphasis => {
                            last_style = None;
                        }
                        pulldown_cmark::Tag::Strong => {
                            last_weight = None;
                        }
                        pulldown_cmark::Tag::Strikethrough => {}
                        pulldown_cmark::Tag::Link(_, _, _) => {}
                        pulldown_cmark::Tag::Image(_, _, _) => {}
                    }
                }
                pulldown_cmark::Event::Text(t) => {
                    font_desc.set_family("sans");
                    layout.set_text(t.as_ref());
                    cairo.move_to(indent, line);
                    line += line_height;
                }
                pulldown_cmark::Event::Code(t) => {
                    font_desc.set_family("mono");
                    layout.set_text(t.as_ref());
                    cairo.move_to(indent, line);
                    line += line_height;
                }
                pulldown_cmark::Event::Html(_) => {}
                pulldown_cmark::Event::FootnoteReference(_) => {}
                pulldown_cmark::Event::SoftBreak => {}
                pulldown_cmark::Event::HardBreak => {}
                pulldown_cmark::Event::Rule => {}
                pulldown_cmark::Event::TaskListMarker(_) => {}
            }
            if let Some(fam) = last_family {
                font_desc.set_family(fam);
            }
            if let Some(size) = last_size {
                font_desc.set_size(size * pango::SCALE);
            }
            if let Some(style) = last_style {
                font_desc.set_style(style);
            }
            if let Some(weight) = last_weight {
                font_desc.set_weight(weight);
            }
            layout.set_font_description(Some(&font_desc));
            pangocairo::functions::show_layout(cairo, &layout);
        }
        
        Ok(())
    }
}

plugin_entry!("zathura-markdown-rs", PluginType, ["text/markdown"]);