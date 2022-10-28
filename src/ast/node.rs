pub struct Node<'a> {
    pub op: i32,
    pub val: String,
    pub sym: IdList,
    pub fnum: i32,
    pub line_no: i32,
    pub left: &'a Node<'a>,
    pub right: &'a Node<'a>,
    pub next: &'a Node<'a>
}

pub struct IdList {

}