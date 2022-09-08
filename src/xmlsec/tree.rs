//!
//! XmlSec tree module
//!

use std::ptr::null;

use crate::bindings;


pub fn xml_sec_add_ids(doc: &mut super::XmlDocument, node: &mut super::XmlNode, ids: &[&str]) {

    unsafe {
        let mut list: Vec<_> = ids.iter().map(|id| id.as_bytes().as_ptr()).chain([null()].into_iter()).collect();
        bindings::xmlSecAddIDs(
            doc.doc_ptr() as *mut bindings::xmlDoc,
            node.node_ptr() as *mut bindings::xmlNode,
            list.as_mut_ptr(),
        );
    }
}
