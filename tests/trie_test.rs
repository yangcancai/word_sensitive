///-------------------------------------------------------------------
/// @author yangcancai

/// Copyright (c) 2021 by yangcancai(yangcancai0112@gmail.com), All Rights Reserved.
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///       https://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
///

/// @doc
///
/// @end
/// Created : 2021-06-03T04:18:52+00:00
///-------------------------------------------------------------------
use word_sensitive::trie;
use std::{fs::File, io::Read};
#[test]
fn add() {
    let mut tree = trie::Trie::default();
    tree.add_key_word(vec![0]);
    if tree.root.borrow().val != None {
        assert_eq!(true, false);
    }
    assert_eq!(tree.root.borrow().children[&0].borrow().val, Some(0));
    assert_eq!(tree.root.borrow().children[&0].borrow().children.len(), 0);
    tree.add_key_word(vec![0, 1]);
    tree.build();
    let node = tree.root.borrow_mut();
    assert_eq!(node.children[&0].borrow().val, Some(0));
    assert_eq!(node.children[&0].borrow().children.len(), 1);
    assert_eq!(
        node.children[&0].borrow().children[&1].borrow().val,
        Some(1)
    );
    assert_eq!(node.key_word_len, Vec::new());
    assert_eq!(node.children[&0].borrow().key_word_len, vec![1]);
    assert_eq!(
        node.children[&0].borrow().children[&1]
            .borrow()
            .key_word_len,
        vec![2]
    );
}
#[test]
fn query() {
    let mut tree = trie::Trie::default();
    tree.add_key_word(vec![0, 1, 2]);
    tree.add_key_word(vec![1, 2]);
    tree.add_key_word(vec![1, 2, 3]);
    tree.add_key_word(vec![3, 4, 5]);
    assert_eq!(
        tree.root.borrow().children[&0].borrow().key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&0].borrow().children[&1]
            .borrow()
            .key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&0].borrow().children[&1]
            .borrow()
            .children[&2]
            .borrow()
            .key_word_len,
        vec![3]
    );

    assert_eq!(
        tree.root.borrow().children[&1].borrow().key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&1].borrow().children[&2]
            .borrow()
            .key_word_len,
        vec![2]
    );
    assert_eq!(
        tree.root.borrow().children[&1].borrow().children[&2]
            .borrow()
            .children[&3]
            .borrow()
            .key_word_len,
        vec![3]
    );

    assert_eq!(
        tree.root.borrow().children[&3].borrow().key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&3].borrow().children[&4]
            .borrow()
            .key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&3].borrow().children[&4]
            .borrow()
            .children[&5]
            .borrow()
            .key_word_len,
        vec![3]
    );

    tree.build();
    // key_word_len
    assert_eq!(
        tree.root.borrow().children[&0].borrow().key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&0].borrow().children[&1]
            .borrow()
            .key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&0].borrow().children[&1]
            .borrow()
            .children[&2]
            .borrow()
            .key_word_len,
        vec![3, 2]
    );

    assert_eq!(
        tree.root.borrow().children[&1].borrow().key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&1].borrow().children[&2]
            .borrow()
            .key_word_len,
        vec![2]
    );
    assert_eq!(
        tree.root.borrow().children[&1].borrow().children[&2]
            .borrow()
            .children[&3]
            .borrow()
            .key_word_len,
        vec![3]
    );

    assert_eq!(
        tree.root.borrow().children[&3].borrow().key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&3].borrow().children[&4]
            .borrow()
            .key_word_len,
        vec![]
    );
    assert_eq!(
        tree.root.borrow().children[&3].borrow().children[&4]
            .borrow()
            .children[&5]
            .borrow()
            .key_word_len,
        vec![3]
    );

    // fail
    assert_eq!(
        tree.root.borrow().children[&0]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        None
    );
    assert_eq!(
        tree.root.borrow().children[&0].borrow().children[&1]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        Some(1)
    );
    assert_eq!(
        tree.root.borrow().children[&0].borrow().children[&1]
            .borrow()
            .children[&2]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        Some(2)
    );

    assert_eq!(
        tree.root.borrow().children[&1]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        None
    );
    assert_eq!(
        tree.root.borrow().children[&1].borrow().children[&2]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        None
    );
    assert_eq!(
        tree.root.borrow().children[&1].borrow().children[&2]
            .borrow()
            .children[&3]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        Some(3)
    );

    assert_eq!(
        tree.root.borrow().children[&3]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        None
    );
    assert_eq!(
        tree.root.borrow().children[&3].borrow().children[&4]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        None
    );
    assert_eq!(
        tree.root.borrow().children[&3].borrow().children[&4]
            .borrow()
            .children[&5]
            .borrow()
            .fail
            .upgrade()
            .unwrap()
            .borrow()
            .val,
        None
    );

    let r = tree.query(&[1, 2, 3]);
    assert_eq!(r[0], &[1, 2]);
    assert_eq!(r[1], &[1, 2, 3]);
    let r = tree.query(&[0, 1, 2]);
    assert_eq!(r[0], &[0, 1, 2]);
    assert_eq!(r[1], &[1, 2]);
    let r = tree.query(&[3, 4, 5]);
    assert_eq!(r[0], &[3, 4, 5]);

    let r = tree.query(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(r[0], &[0, 1, 2]);
    assert_eq!(r[1], &[1, 2]);
    assert_eq!(r[2], &[1, 2, 3]);
    assert_eq!(r[3], &[3, 4, 5]);

    let r = tree.query(&[7, 8, 9, 0, 1, 2, 7, 8, 3, 4, 5]);
    assert_eq!(r[0], &[0, 1, 2]);
    assert_eq!(r[1], &[1, 2]);
    assert_eq!(r[2], &[3, 4, 5]);

   let r = tree.query(&[7, 8, 9, 0, 1, 2, 7, 8, 3, 4, 5, 6]);
    assert_eq!(r[0], &[0, 1, 2]);
    assert_eq!(r[1], &[1, 2]);
    assert_eq!(r[2], &[3, 4, 5]);
    // chinese
    let mut tree = trie::Trie::default();
    tree.add_key_word("中国人".as_bytes().to_vec());
    tree.add_key_word("abc".as_bytes().to_vec());
   let text = "abc你好,中国人";
   tree.build();
   let r = tree.query(text.as_bytes().as_ref());
    assert_eq!(r[0], "abc".as_bytes().as_ref());
    assert_eq!(r[1], "中国人".as_bytes().as_ref());
}
#[test]
fn key_words_from_file(){
    let mut tree = trie::Trie::default();
    let key_words = key_words().unwrap();
    key_words.iter().for_each(|x| tree.add_key_word(x.as_bytes().to_vec()));
    tree.build();
    let r = tree.query("回民吃猪肉".as_bytes().as_ref());
    assert_eq!(r[0], "回民".as_bytes().as_ref());
    assert_eq!(r[1], "回民吃猪肉".as_bytes().as_ref());
}
fn key_words() -> std::io::Result<Vec<String>>{
    let mut file = File::open("key_words/keywords.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.split("\n").map(|x|x.to_string()).collect::<Vec<String>>())
}