use word_sensitive::Trie;
use word_sensitive::trie::NodeExt;
fn main() {
    let mut tree = Trie::<usize>::default();
    tree.add_key_word("aaa".as_bytes().to_vec());
    tree.add_key_word("aab".as_bytes().to_vec());
    tree.add_key_word("aac".as_bytes().to_vec());
    tree.build();
    tree.add_key_word("aba".as_bytes().to_vec());
    tree.add_key_word("abb".as_bytes().to_vec());
    tree.add_key_word("abc".as_bytes().to_vec());
    tree.build();
    tree.add_key_word("aca".as_bytes().to_vec());
    tree.add_key_word("acb".as_bytes().to_vec());
    tree.add_key_word("acc".as_bytes().to_vec());
    tree.build();
    tree.add_key_word("baa".as_bytes().to_vec());
    tree.add_key_word("bab".as_bytes().to_vec());
    tree.add_key_word("bac".as_bytes().to_vec());
    tree.build();
    tree.add_key_word("bba".as_bytes().to_vec());
    tree.add_key_word("bbb".as_bytes().to_vec());
    tree.add_key_word("bbc".as_bytes().to_vec());
    tree.build();
    tree.add_key_word("bba".as_bytes().to_vec());
    tree.add_key_word("bca".as_bytes().to_vec());
    tree.add_key_word("bcb".as_bytes().to_vec());
    tree.add_key_word("bcc".as_bytes().to_vec());
    tree.build();
    tree.add_key_word("caa".as_bytes().to_vec());
    tree.add_key_word("cab".as_bytes().to_vec());
    tree.add_key_word("cac".as_bytes().to_vec());

    tree.build();
    tree.add_key_word("cba".as_bytes().to_vec());
    tree.add_key_word("cbb".as_bytes().to_vec());
    tree.add_key_word("cbc".as_bytes().to_vec());

    tree.build();
    tree.add_key_word("cca".as_bytes().to_vec());
    tree.add_key_word("ccb".as_bytes().to_vec());
    tree.add_key_word("ccc".as_bytes().to_vec());
    tree.build();
    let mut tree = Trie::default();

    #[derive(Clone)]
    struct Ext{
        cate: usize,
        len: usize,
        weight: usize
    }
    impl NodeExt for Ext{
        fn get_len(&self) -> usize{
            self.len
        }
        fn get_weight(&self) -> usize {
            self.weight
        }
        fn get_cate(&self) -> usize{
            self.cate
        }
        fn eq(&self, other: &Self) -> bool{
            self.len == other.len
        }
    }
    tree.add_key_word_ext("abc".as_bytes().to_vec(), Ext{cate:1, len: 3, weight: 1});
    tree.add_key_word_ext("bc".as_bytes().to_vec(), Ext{cate:2, len: 2, weight: 10});
    tree.build();
    let r = tree.query_cate_weight("abc".as_bytes().as_ref());
    assert_eq!(r[&1], 1);
    assert_eq!(r[&2], 10);
    assert_eq!(tree.query_total_weight("abc".as_bytes().as_ref()), 11);
    println!("Hello, world!");
}
