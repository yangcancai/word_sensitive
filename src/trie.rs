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
use core::ptr::NonNull;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
type Value = u8;
type NodePtr<T> = NonNull<Node<T>>;
pub trait NodeExt {
    fn get_len(&self) -> usize;
    fn eq(&self, other: &Self) -> bool;
    fn get_weight(&self) -> usize;
    fn get_cate(&self) -> usize;
}
impl NodeExt for usize {
    fn get_len(&self) -> usize {
        *self
    }
    fn eq(&self, other: &usize) -> bool {
        *self == *other
    }
    fn get_weight(&self) -> usize {
        1
    }
    fn get_cate(&self) -> usize {
        1
    }
}
#[derive(Debug)]
pub struct Node<T> {
    // fail pointer
    pub fail: Option<NodePtr<T>>,
    // all children node
    pub children: HashMap<Value, NodePtr<T>>,
    // current node value
    pub val: Option<Value>,
    // all path
    pub ext: Vec<T>,
}
pub struct Trie<T> {
    pub root: NodePtr<T>,
    marker: PhantomData<Box<Node<T>>>,
}
impl<T> Default for Node<T> {
    fn default() -> Self {
        Node {
            fail: None,
            children: HashMap::new(),
            val: None,
            ext: Vec::new(),
        }
    }
}
impl<T: NodeExt> Node<T> {
    pub fn new(val: Value) -> NodePtr<T> {
        Box::leak(Box::new(Node {
            fail: None,
            children: HashMap::new(),
            val: Some(val),
            ext: Vec::<T>::new(),
        }))
        .into()
    }
    pub fn push(&mut self, element: T) {
        for e in self.ext.iter_mut() {
            if e.eq(&element) {
                *e = element;
                return;
            }
        }
        self.ext.push(element);
    }
}
impl<T> Default for Trie<T> {
    fn default() -> Self {
        Trie {
            root: Box::leak(Box::new(Node::default())).into(),
            marker: PhantomData,
        }
    }
}
impl Trie<usize> {
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
        contents
            .split('\n')
            .for_each(|x| self.add_key_word(x.as_bytes().to_vec()));
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
        let len = key_word.len();
        self.add_key_word_ext(key_word, len)
    }
}
impl<T: NodeExt + Clone> Trie<T> {
    pub fn add_key_word_ext(&mut self, key_word: Vec<Value>, ext: T) {
        if key_word.is_empty() {
            return;
        }
        let mut cur = self.root;
        for (_, val) in key_word.iter().enumerate() {
            unsafe {
                let temp = (*cur.as_ptr())
                    .children
                    .entry(*val)
                    .or_insert_with(|| Node::<T>::new(*val));
                cur = *temp;
            }
        }
        // Append key_word_len
        unsafe {
            (*cur.as_ptr()).push(ext);
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
        unsafe {
            let first_level_fail = self.root;
            for (_i, child) in (*self.root.as_ptr()).children.iter() {
                (*child.as_ptr()).fail = Some(first_level_fail);
                queue.push_back(child);
            }
            while let Some(node) = queue.pop_front() {
                let cur = node;
                // Find fail for all children
                for (i, child) in (*cur.as_ptr()).children.iter() {
                    // Father fail
                    let mut fafail = (*cur.as_ptr()).fail;
                    // Find father fail until fafail is none or fafail.children[i] is not none
                    while fafail.is_some() && (*fafail.unwrap().as_ptr()).children.get(&i).is_none()
                    {
                        fafail = (*fafail.unwrap().as_ptr()).fail;
                    }
                    let temp = match fafail {
                        // Fafail is none ,we knonw fafail is root
                        None => Some(self.root),
                        // Else fafail.children[i] will be child fail poiter
                        Some(v) => {
                            let children_i = (*v.as_ptr()).children.get(&i).unwrap();
                            (*children_i.as_ptr()).ext.iter().for_each(|x| {
                                (*child.as_ptr()).push(x.clone());
                            });
                            // Append key_word_len for other key_word
                            Some(*(*v.as_ptr()).children.get(&i).unwrap())
                        }
                    };
                    (*child.as_ptr()).fail = temp;
                    queue.push_back(child)
                }
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
        self.query_ext(text)
            .iter()
            .map(|(i, x)| &text[*i - x.get_len()..*i])
            .collect()
    }
    /// Query total weight for text
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use word_sensitive::trie;
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word("abc".as_bytes().to_vec());
    /// tree.add_key_word("bc".as_bytes().to_vec());
    /// tree.build();
    /// assert_eq!(2, tree.query_total_weight("abc".as_bytes().as_ref()));
    ///
    ///
    /// #[derive(Clone)]
    /// struct Ext {
    /// cate: usize,
    /// weight: usize,
    /// len: usize,
    /// }
    /// impl trie::NodeExt for Ext {
    /// fn get_len(&self) -> usize {
    ///     self.len
    /// }
    /// fn eq(&self, other: &Self) -> bool {
    ///     self.len == other.len
    /// }
    /// fn get_weight(&self) -> usize {
    ///     self.weight
    /// }
    /// fn get_cate(&self) -> usize {
    ///     self.cate
    /// }
    /// } 
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word_ext(
    ///     "abc".as_bytes().to_vec(),
    ///     Ext {
    ///         cate: 1,
    ///         weight: 2,
    ///         len: 3,
    ///     },
    /// );
    /// tree.add_key_word_ext(
    ///     "bc".as_bytes().to_vec(),
    ///     Ext {
    ///         cate: 1,
    ///         weight: 1,
    ///         len: 2,
    ///     },
    /// );
    /// tree.build();
    /// assert_eq!(3, tree.query_total_weight("abc".as_bytes().as_ref()));
    /// ```
    pub fn query_total_weight(&self, text: &[Value]) -> usize {
        self.query_ext(text)
            .iter()
            .fold(0usize, |acc, (_, x)| acc + x.get_weight())
    }
    /// Query Category weight for text
    ///
    /// # Examples
    ///
    /// ```
    /// use word_sensitive::trie;
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word("abc".as_bytes().to_vec());
    /// tree.add_key_word("bc".as_bytes().to_vec());
    /// tree.build();
    /// assert_eq!(
    ///     Some(&2),
    ///     tree.query_cate_weight("abc".as_bytes().as_ref()).get(&1)
    /// );
    /// #[derive(Clone)]
    /// struct Ext {
    /// cate: usize,
    /// weight: usize,
    /// len: usize,
    /// }
    /// impl trie::NodeExt for Ext {
    /// fn get_len(&self) -> usize {
    ///     self.len
    /// }
    /// fn eq(&self, other: &Self) -> bool {
    ///     self.len == other.len
    /// }
    /// fn get_weight(&self) -> usize {
    ///     self.weight
    /// }
    /// fn get_cate(&self) -> usize {
    ///     self.cate
    /// }
    /// }
    /// let mut tree = trie::Trie::default();
    /// tree.add_key_word_ext(
    ///     "abc".as_bytes().to_vec(),
    ///     Ext {
    ///         cate: 1,
    ///         weight: 2,
    ///         len: 3,
    ///     },
    /// );
    /// tree.add_key_word_ext(
    ///     "bc".as_bytes().to_vec(),
    ///     Ext {
    ///         cate: 1,
    ///         weight: 1,
    ///         len: 2,
    ///     },
    /// );
    /// tree.build();
    /// assert_eq!(
    ///     Some(&3),
    ///     tree.query_cate_weight("abc".as_bytes().as_ref()).get(&1)
    /// );

    /// tree.add_key_word_ext(
    ///     "bc".as_bytes().to_vec(),
    ///     Ext {
    ///         cate: 2,
    ///         weight: 1,
    ///         len: 2,
    ///     },
    /// );
    /// tree.build();
    /// tree.add_key_word_ext(
    ///     "ab".as_bytes().to_vec(),
    ///     Ext {
    ///         cate: 2,
    ///         weight: 1,
    ///         len: 2,
    ///     },
    /// );
    /// tree.build();
    /// assert_eq!(
    ///     Some(&2),
    ///     tree.query_cate_weight("abc".as_bytes().as_ref()).get(&1)
    /// );
    /// assert_eq!(
    ///     Some(&2),
    ///     tree.query_cate_weight("abc".as_bytes().as_ref()).get(&2)
    /// );

    /// ```
    pub fn query_cate_weight(&self, text: &[Value]) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        self.query_ext(text).iter().for_each(|(_i, x)| {
            if let Some(e) = result.get_mut(&x.get_cate()) {
                *e += x.get_weight();
            } else {
                result.insert(x.get_cate(), x.get_weight());
            }
        });
        result
    }
    pub fn query_ext<'a>(&self, text: &'a [Value]) -> Vec<(usize, &T)> {
        let mut result = Vec::new();
        let mut cur = Some(self.root);
        for (i, e) in text.iter().enumerate() {
            unsafe {
                if let Some(v) = cur {
                    // Find  child, child is none, let fail.children[e]
                    // until fail.children[e] is not none or fail equal none
                    let mut child = v;
                    while (*child.as_ptr()).children.get(e).is_none() {
                        if (*child.as_ptr()).fail.is_none() {
                            child = self.root;
                            break;
                        }
                        child = (*child.as_ptr()).fail.unwrap();
                    }
                    cur = match (*child.as_ptr()).children.get(e) {
                        None => Some(self.root),
                        Some(child) => {
                            result.append(
                                &mut (*child.as_ptr()).ext.iter().map(|x| (i + 1, x)).collect(),
                            );
                            Some(*child)
                        }
                    }
                } else {
                    cur = Some(self.root);
                }
            }
        }
        result
    }
}
unsafe impl<T> Send for Trie<T> {}

unsafe impl<T> Sync for Trie<T> {}
