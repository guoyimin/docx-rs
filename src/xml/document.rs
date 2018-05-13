use quick_xml::events::Event;
use std::collections::LinkedList;
use std::default::Default;

use body::Para;
use events_list::EventListExt;
use schema::SCHEMA_MAIN;
use xml::Xml;

static DOCUMENT_NAMESPACES: [(&str, &str); 1] = [("xmlns:w", SCHEMA_MAIN)];

#[derive(Debug)]
pub struct DocumentXml<'a> {
  body: Vec<Para<'a>>,
}

impl<'a> DocumentXml<'a> {
  pub fn add_para(&mut self, para: Para<'a>) {
    self.body.push(para);
  }
}

impl<'a> Default for DocumentXml<'a> {
  fn default() -> DocumentXml<'a> {
    DocumentXml { body: Vec::new() }
  }
}

impl<'a> Xml<'a> for DocumentXml<'a> {
  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for para in &self.body {
      events.append(&mut para.events());
    }

    events
      .warp_tag("w:body")
      .warp_attrs_tag("w:document", DOCUMENT_NAMESPACES.to_vec());

    events
  }
}
