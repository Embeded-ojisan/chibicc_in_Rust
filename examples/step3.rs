use std::collections::LinkedList;

enum TokenKind {
    TK_RESERVED,
    TK_NUM,
    TK_EOF
}

struct Token {
    kind: TokenKind,
    next: Link,
    val: usize,
    str: u8,
}

type Link = Option<Box<Token>>;

fn error()
{
    ;
}

fn consume() -> bool
{
    ;
}
