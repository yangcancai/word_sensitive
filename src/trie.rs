//-------------------------------------------------------------------
// @author yangcanc//
// Copyright (c) 2021 by yangcancai(yangcancai0112@gmail.com), All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// @doc
//
// @end
// Created : 2021-06-03T03:59:08+00:00
//-------------------------------------------------------------------
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::sync::Weak;
use std::sync::Arc;
type Value = u8;
type NodePtr = Arc<RefCell<Node>>;
type WeakPtr = Weak<RefCell<Node>>;
#[derive(Debug)]
pub struct Node {
    // fail pointer
    pub fail: WeakPtr,
    // all children node
    pub children: HashMap<Value, NodePtr>,
    // current node value
    pub val: Option<Value>,
    // all path
    pub key_word_len: Vec<usize>,
}
pub struct Trie {
    pub root: NodePtr,
}
impl Default for Node {
    fn default() -> Self {
        Node {
            fail: Weak::new(),
            children: HashMap::new(),
            val: None,
            key_word_len: Vec::new(),
        }
    }
}
impl Node {
    pub fn new(val: Value) -> NodePtr {
        Arc::new(RefCell::new(Node {
            fail: Weak::new(),
            children: HashMap::new(),
            val: Some(val),
            key_word_len: Vec::new(),
        }))
    }
}
impl Default for Trie {
    fn default() -> Self {
        Trie {
            root: Arc::new(RefCell::new(Node::default())),
        }
    }
}
impl Trie {
    /// Add keywords from file
    ///
    /// # Examples
    /// 
    ///
    /// ```
    /// use word_sensitive::trie;
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word_from_file("key_words/keywords.txt").unwrap();
    /// tree.build();
    /// let matches = tree.query("回民吃猪肉".as_bytes().as_ref());
    /// assert_eq!(matches[0], "回民".as_bytes().as_ref());
    /// assert_eq!(matches[1], "回民吃猪肉".as_bytes().as_ref());
    /// ```
    pub fn add_key_word_from_file(&mut self, file: &str) -> std::io::Result<()> {
      let mut file = File::open(file)?;
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      contents.split('\n').for_each(|x| self.add_key_word(x.as_bytes().to_vec()));
      Ok(())
    }
    /// Add keyword 
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// use word_sensitive::trie;
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word("abc".as_bytes().to_vec());
    /// tree.add_key_word("bcd".as_bytes().to_vec());
    /// tree.build();
    /// let matches = tree.query("hello, abcd".as_bytes().as_ref());
    /// assert_eq!(matches[0], "abc".as_bytes().as_ref());
    /// assert_eq!(matches[1], "bcd".as_bytes().as_ref());
    /// ```
    pub fn add_key_word(&mut self, key_word: Vec<Value>) {
        let mut cur = self.root.clone();
        for (_, val) in key_word.iter().enumerate() {
            let temp = cur
                .borrow_mut()
                .children
                .entry(*val)
                .or_insert_with(|| Node::new(*val))
                .clone();
            cur = temp;
        }
        // Append key_word_len
        let c = cur.borrow().key_word_len.clone();
        let temp: Vec<&usize> = c.iter().filter(|&x| *x == key_word.len()).collect();
        if temp.is_empty() {
            cur.borrow_mut().key_word_len.push(key_word.len());
        }
    }
    /// Build fail pointer for tree
    ///
    /// # Examples
    ///
    ///
    /// ```
    /// use word_sensitive::trie;
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word("abc".as_bytes().to_vec());
    /// tree.add_key_word("bac".as_bytes().to_vec());
    /// tree.add_key_word("aac".as_bytes().to_vec());
    /// tree.add_key_word("bcd".as_bytes().to_vec());
    /// tree.build();
    /// let matches = tree.query("abcdef".as_bytes().as_ref());
    /// assert_eq!(matches[0], "abc".as_bytes().as_ref()) ;
    /// assert_eq!(matches[1], "bcd".as_bytes().as_ref()) ;
    /// ```
    pub fn build(&mut self) {
        let mut queue = VecDeque::new();
        // First level all child fail poiter is root
        let first_level_fail = self.root.clone();
        for (_i, child) in self.root.borrow().children.iter() {
            child.borrow_mut().fail = Arc::downgrade(&first_level_fail.clone());
            queue.push_back(child.clone());
        }
        while let Some(node) = queue.pop_front() {
            let cur = node.borrow();
            // Find fail for all children
            for (i, child) in cur.children.iter() {
                // Father fail
                let mut fafail = cur.fail.clone();
                // Find father fail until fafail is none or fafail.children[i] is not none
                while fafail.upgrade().is_some()
                    && fafail
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .children
                        .get(&i)
                        .is_none()
                {
                    fafail = fafail.upgrade().unwrap().borrow().fail.clone();
                }
                let temp = match fafail.upgrade() {
                    // Fafail is none ,we knonw fafail is root
                    None => Arc::downgrade(&self.root.clone()),
                    // Else fafail.children[i] will be child fail poiter
                    Some(v) => {
                        let children_i = v.borrow().children.get(&i).unwrap().clone();
                        children_i.borrow().key_word_len.iter().for_each(|&x| {
                            child.borrow_mut().key_word_len.push(x);
                        });
                        // Append key_word_len for other key_word
                        Arc::downgrade(&v.borrow().children.get(&i).unwrap().clone())
                    }
                };
                child.borrow_mut().fail = temp;
                queue.push_back(child.clone())
            }
        }
    }
    /// Query all key_words input text string
    /// 
    /// # Examples 
    ///
    ///
    /// ```
    /// use word_sensitive::trie;
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word("aaa".as_bytes().to_vec());
    /// tree.add_key_word("aab".as_bytes().to_vec());
    /// tree.add_key_word("aac".as_bytes().to_vec());
    /// tree.add_key_word("aba".as_bytes().to_vec());
    /// tree.add_key_word("abb".as_bytes().to_vec());
    /// tree.add_key_word("abc".as_bytes().to_vec());
    /// tree.add_key_word("aca".as_bytes().to_vec());
    /// tree.add_key_word("acb".as_bytes().to_vec());
    /// tree.add_key_word("acc".as_bytes().to_vec());
    /// tree.add_key_word("baa".as_bytes().to_vec());
    /// tree.add_key_word("bab".as_bytes().to_vec());
    /// tree.add_key_word("bac".as_bytes().to_vec());
    /// tree.add_key_word("bba".as_bytes().to_vec());
    /// tree.add_key_word("bbb".as_bytes().to_vec());
    /// tree.add_key_word("bbc".as_bytes().to_vec());
    /// tree.add_key_word("bca".as_bytes().to_vec());
    /// tree.add_key_word("bcb".as_bytes().to_vec());
    /// tree.add_key_word("bcc".as_bytes().to_vec());
    /// tree.add_key_word("caa".as_bytes().to_vec());
    /// tree.add_key_word("cab".as_bytes().to_vec());
    /// tree.add_key_word("cac".as_bytes().to_vec());
    /// tree.add_key_word("cba".as_bytes().to_vec());
    /// tree.add_key_word("cbb".as_bytes().to_vec());
    /// tree.add_key_word("cbc".as_bytes().to_vec());
    /// tree.add_key_word("cca".as_bytes().to_vec());
    /// tree.add_key_word("ccb".as_bytes().to_vec());
    /// tree.add_key_word("ccc".as_bytes().to_vec());
    /// tree.build();
    /// let matches = tree.query("abcabcbcca".as_bytes().as_ref());
    /// assert_eq!(matches[0], "abc".as_bytes().as_ref());
    /// assert_eq!(matches[1], "bca".as_bytes().as_ref());
    /// assert_eq!(matches[2], "cab".as_bytes().as_ref());
    /// assert_eq!(matches[3], "abc".as_bytes().as_ref());
    /// assert_eq!(matches[4], "bcb".as_bytes().as_ref());
    /// assert_eq!(matches[5], "cbc".as_bytes().as_ref());
    /// assert_eq!(matches[6], "bcc".as_bytes().as_ref());
    /// assert_eq!(matches[7], "cca".as_bytes().as_ref());
    /// ```
    pub fn query<'a>(&self, text: &'a [Value]) -> Vec<&'a [Value]> {
        let mut result: Vec<&[Value]> = Vec::new();
        let mut cur = Arc::downgrade(&self.root);
        for (i, e) in text.iter().enumerate() {
            if let Some(v) = cur.upgrade() {
                // Find  child, child is none, let fail.children[e]
                // until fail.children[e] is not none or fail equal none
                let mut child = Arc::downgrade(&v.clone());
                while child.upgrade().unwrap().borrow().children.get(e).is_none() {
                    if child.upgrade().unwrap().borrow().fail.upgrade().is_none() {
                        child = Arc::downgrade(&self.root.clone());
                        break;
                    }
                    child = child.upgrade().unwrap().borrow().fail.clone();
                }
                cur = match child.upgrade().unwrap().borrow().children.get(e) {
                    None => Arc::downgrade(&self.root),
                    Some(child) => {
                        result.append(
                            &mut child
                                .borrow()
                                .key_word_len
                                .iter()
                                .map(|x| &text[i + 1 - x..i + 1])
                                .collect(),
                        );
                        Arc::downgrade(&child)
                    }
                }
            } else {
                cur = Arc::downgrade(&self.root);
            }
        }
        result
    }
}
