use word_sensitive::trie;
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
use word_sensitive::Trie;
#[test]
fn add() {
    let mut tree = Trie::default();
    tree.add_key_word(vec![0]);
    unsafe {
        if (*tree.root.as_ptr()).val != None {
            assert_eq!(true, false);
        }
        assert_eq!((*(*tree.root.as_ptr()).children[&0].as_ptr()).val, Some(0));
        assert_eq!(
            (*(*tree.root.as_ptr()).children[&0].as_ptr())
                .children
                .len(),
            0
        );
        tree.add_key_word(vec![0, 1]);
        tree.build();
        assert_eq!((*(*tree.root.as_ptr()).children[&0].as_ptr()).val, Some(0));
        assert_eq!(
            (*(*tree.root.as_ptr()).children[&0].as_ptr())
                .children
                .len(),
            1
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).val,
            Some(1)
        );
        assert_eq!((*tree.root.as_ptr()).key_word_len, Vec::new());
        assert_eq!(
            (*(*tree.root.as_ptr()).children[&0].as_ptr()).key_word_len,
            vec![1]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).key_word_len,
            vec![2]
        );
    }
}
#[test]
fn query() {
    let mut tree = trie::Trie::default();
    tree.add_key_word(vec![0, 1, 2]);
    tree.add_key_word(vec![1, 2]);
    tree.add_key_word(vec![1, 2, 3]);
    tree.add_key_word(vec![3, 4, 5]);
    unsafe {
        assert_eq!(
            (*(*tree.root.as_ptr()).children[&0].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).children[&2]
                .as_ptr())
            .key_word_len,
            vec![3]
        );

        assert_eq!(
            (*(*tree.root.as_ptr()).children[&1].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&1].as_ptr()).children[&2].as_ptr()).key_word_len,
            vec![2]
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&1].as_ptr()).children[&2].as_ptr()).children[&3]
                .as_ptr())
            .key_word_len,
            vec![3]
        );

        assert_eq!(
            (*(*tree.root.as_ptr()).children[&3].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&3].as_ptr()).children[&4].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&3].as_ptr()).children[&4].as_ptr()).children[&5]
                .as_ptr())
            .key_word_len,
            vec![3]
        );

        tree.build();
        // key_word_len
        assert_eq!(
            (*(*tree.root.as_ptr()).children[&0].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).children[&2]
                .as_ptr())
            .key_word_len,
            vec![3, 2]
        );

        assert_eq!(
            (*(*tree.root.as_ptr()).children[&1].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&1].as_ptr()).children[&2].as_ptr()).key_word_len,
            vec![2]
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&1].as_ptr()).children[&2].as_ptr()).children[&3]
                .as_ptr())
            .key_word_len,
            vec![3]
        );

        assert_eq!(
            (*(*tree.root.as_ptr()).children[&3].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&3].as_ptr()).children[&4].as_ptr()).key_word_len,
            vec![]
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&3].as_ptr()).children[&4].as_ptr()).children[&5]
                .as_ptr())
            .key_word_len,
            vec![3]
        );

        // fail
        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&0].as_ptr())
                .fail
                .unwrap()
                .as_ptr())
            .val,
            None
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr())
                .fail
                .unwrap()
                .as_ptr())
            .val,
            Some(1)
        );
        assert_eq!(
            (*(*(*(*(*tree.root.as_ptr()).children[&0].as_ptr()).children[&1].as_ptr()).children
                [&2]
                .as_ptr())
            .fail
            .unwrap()
            .as_ptr())
            .val,
            Some(2)
        );

        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&1].as_ptr())
                .fail
                .unwrap()
                .as_ptr())
            .val,
            None
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&1].as_ptr()).children[&2].as_ptr())
                .fail
                .unwrap()
                .as_ptr())
            .val,
            None
        );
        assert_eq!(
            (*(*(*(*(*tree.root.as_ptr()).children[&1].as_ptr()).children[&2].as_ptr()).children
                [&3]
                .as_ptr())
            .fail
            .unwrap()
            .as_ptr())
            .val,
            Some(3)
        );

        assert_eq!(
            (*(*(*tree.root.as_ptr()).children[&3].as_ptr())
                .fail
                .unwrap()
                .as_ptr())
            .val,
            None
        );
        assert_eq!(
            (*(*(*(*tree.root.as_ptr()).children[&3].as_ptr()).children[&4].as_ptr())
                .fail
                .unwrap()
                .as_ptr())
            .val,
            None
        );
        assert_eq!(
            (*(*(*(*(*tree.root.as_ptr()).children[&3].as_ptr()).children[&4].as_ptr()).children
                [&5]
                .as_ptr())
            .fail
            .unwrap()
            .as_ptr())
            .val,
            None
        );
    }

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
fn key_words_from_file() {
    let mut tree = trie::Trie::default();
    tree.add_key_word_from_file("key_words/keywords.txt")
        .unwrap();
    tree.build();
    let r = tree.query("回民吃猪肉".as_bytes().as_ref());
    assert_eq!(r[0], "回民".as_bytes().as_ref());
    assert_eq!(r[1], "回民吃猪肉".as_bytes().as_ref());
}
#[test]
fn specail_build() {
    let mut tree = trie::Trie::default();
    tree.add_key_word("aaa".as_bytes().to_vec());
    tree.add_key_word("aab".as_bytes().to_vec());
    tree.add_key_word("aac".as_bytes().to_vec());

    tree.add_key_word("aba".as_bytes().to_vec());
    tree.add_key_word("abb".as_bytes().to_vec());
    tree.add_key_word("abc".as_bytes().to_vec());

    tree.add_key_word("aca".as_bytes().to_vec());
    tree.add_key_word("acb".as_bytes().to_vec());
    tree.add_key_word("acc".as_bytes().to_vec());

    tree.add_key_word("baa".as_bytes().to_vec());
    tree.add_key_word("bab".as_bytes().to_vec());
    tree.add_key_word("bac".as_bytes().to_vec());

    tree.add_key_word("bba".as_bytes().to_vec());
    tree.add_key_word("bbb".as_bytes().to_vec());
    tree.add_key_word("bbc".as_bytes().to_vec());

    tree.add_key_word("bca".as_bytes().to_vec());
    tree.add_key_word("bcb".as_bytes().to_vec());
    tree.add_key_word("bcc".as_bytes().to_vec());

    tree.add_key_word("caa".as_bytes().to_vec());
    tree.add_key_word("cab".as_bytes().to_vec());
    tree.add_key_word("cac".as_bytes().to_vec());

    tree.add_key_word("cba".as_bytes().to_vec());
    tree.add_key_word("cbb".as_bytes().to_vec());
    tree.add_key_word("cbc".as_bytes().to_vec());

    tree.add_key_word("cca".as_bytes().to_vec());
    tree.add_key_word("ccb".as_bytes().to_vec());
    tree.add_key_word("ccc".as_bytes().to_vec());

    tree.build();
    let r = tree.query("abccbabacaacabc".as_ref());
    assert_eq!(r[0], "abc".as_bytes().as_ref());
    assert_eq!(r[1], "bcc".as_bytes().as_ref());
    assert_eq!(r[2], "ccb".as_bytes().as_ref());
    assert_eq!(r[3], "cba".as_bytes().as_ref());
    assert_eq!(r[4], "bab".as_bytes().as_ref());
    assert_eq!(r[5], "aba".as_bytes().as_ref());
    assert_eq!(r[6], "bac".as_bytes().as_ref());
    assert_eq!(r[7], "aca".as_bytes().as_ref());
    assert_eq!(r[8], "caa".as_bytes().as_ref());
    assert_eq!(r[9], "aac".as_bytes().as_ref());
    assert_eq!(r[10], "aca".as_bytes().as_ref());
    assert_eq!(r[11], "cab".as_bytes().as_ref());
    assert_eq!(r[12], "abc".as_bytes().as_ref());
}
