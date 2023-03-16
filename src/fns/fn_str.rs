///Command string function.

use std::error::Error;
type Result<T>=::std::result::Result<T, Box<dyn Error>>;

///First char of first word to Capitalize
///A mnemonic node
///
/// Example:
///```rust
/// use jay_lib::fns::fn_str::capitalize_first;
/// let s="hello word";
///
/// assert_eq!("Hello word",capitalize_first(s));
///
/// ```
pub fn capitalize_first(input: &str) -> String {
    let s=input.trim();
    let mut c = s.chars();
    match c.next() {
        None => return String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

///`Vec<u8>` to `Vec<char>`
///A mnemonic node
///
/// Example:
///```rust
///   //SQLite format 3
///    use jay_lib::fns::fn_str::u8_chars;
///    let codepoint_array: Vec<u8> = "\n\0\rtest\00x08".into();
///    let codepoint_array = vec![23, 33, 43, 53, 63, 74, 84, 192, 241];
///     let codepoints: Vec<char> = codepoint_array.into_iter().map(char::from).collect();
///     let codepoint_array = b"SQLite format 3"; // + b'\0';
///     let mut vc = codepoint_array.to_vec();
///     vc.push(b'\0');
///     let codepoints =u8_chars(vc);
///     println!("{:?},{}", codepoints, codepoints.len());
/// ```
pub fn u8_chars(v: Vec<u8>) -> Vec<char> {
    let r: Vec<char> = v.into_iter().map(char::from).collect();
    r
}

///`Vec<u8>` to String
///A mnemonic node
///```rust
/// use jay_lib::fns::fn_str::v8_string;
/// let mut buffer: [u8; 16] = [0; 16];
///     let v = buffer.to_vec();
///     let s = v8_string(v);
///     println!("{}", s.len());
/// ```
pub fn v8_string(v: Vec<u8>) -> String {
    v.into_iter().map(char::from).collect()
}

///It is get first word form str.
///Just a mnemonic node to remember it.
/// Example:
///```rust
/// use jay_lib::fns::fn_str::first_word;
///if let Some(subject)=first_word("abc def"){
///	  assert_eq!("abc".to_string(),subject)
///}
///assert_eq!(None,first_word(""));
///```
pub fn first_word<S:AsRef<str>>(s: S) -> Option<String> {
    let s = s.as_ref().to_string();
    if let Some(sub)=s.split_whitespace().next(){
        let s=sub.chars().filter(|c|c.is_alphabetic()||*c=='\'').collect::<String>() ;
        Some(s)
    }else { None }
}

///It is get last word form str.
///Just to remember it.
/// Example:
///``` rust
/// use jay_lib::fns::fn_str::last_word;
///if let Some(subject)=last_word("abc def"){
///	  assert_eq!("def".to_string(),subject)
///}
///assert_eq!(None,last_word(""));
///```
///
pub fn last_word<S:AsRef<str>>(s: S) -> Option<String> {
    let s = s.as_ref().to_string();
    if let Some(sub)=s.split_whitespace().last(){
        Some(sub.to_string())
    }else { None }
}

///Command get middle subject str.
/// Example:
/// ```rust
/// use jay_lib::fns::fn_str;
///     let s=r#"<DT><A HREF="http://web.stanford.edu/class/cs97si/" ADD_DATE="1606284605" ICON="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAADIUlEQVQ4jV1SPWxTZxQ9937PPP/FP1JenxNDUAsqIk5FHFoqtYmCKiwVwsJQKhZABYaMLVGHdmg21ECrMiChdEHqUomxRLAUWxAgmVCKZBW1yBirpHET2+EljoP9fbcDcUpzlruce3XOuYewBb+cO/ed8bzP19bWjGjNAEBKmUAgwCoS+eHo5OQXr/PVxqTxgwetL48c+bZRKp1/ks2apcePOeS69NLzqHT3LtWKRROORj84lcl0vB0K3c49fSoAwOMAA5BUZ2fGVCpjf2Szmv1+EoDSZ8/KvtOnxRhDKhCgJ7mcRqVyPuW6GQAyDrDVCxAAQOuAt7Ji2LIgzSbZ4bA4fX0krZYo2zattTVtjDHe4mJLxeN+APgGgJUHBACMiFHMJCIAs+weGeFAPC52JEL7Tpyg53fusGKGk0jgH8/bBgDXAbK2higAiJki3d3ij8UgWktyaCgvfv+0su0CKxWsTE39BgCfAOZ/B4RIoLUEHQe7Dh8my7ZhWi2EXDcq9Xr/S89b+vjata/bfAKENzNQStk+HxutTXN1FbOXL9NquUyNWo3mZ2a2z8/MvN8qlb6aGh39+aLrhrCxx8cBAwCN9fUZ5ThzPamUr1YscueePRJ0HPiCQdSrVVkqFvXv9+419eLipz2Dgx+1v8Bt26du3PjrWa02GBsY+LXv2LGVaE+PUT6f+GMxvHnoEDqSSSZmrnueIWYLAHoB4rb9i677RuzFi/eWy+UJ8vl+iu3cycRMYoxUCwVqNRqv8iUiMUYAIA9Iu0joSqc/TOzde7u1vHwzceDAZ9sHB1EtFEDMlD5zBh1dXdDN5obo/7BZJDscpoVHj3RpepoiyaQ9eeWKvHPypCTSaSrPzcnukRGaf/jQaK1h0SvlvQBtFkmI6hHXVduCQf3nrVumVq3yu+GwPH/wQG5euEDJHTuMsiyEo1FuMtfbFlRuI0SkUoWBzk67Ix4fqhQKJtnfT5lLlyjgODSfzcp6pSJvDQ8rO5mcaHjej9fzeeQAoa1NnBodnVhfWBhLDg/r7v37CQCe3b8vf8/OKttxvj969erY6/x/AcVGWaUgUfh8AAAAAElFTkSuQmCC">CS 97SI: Introduction to Programming Contests</A>"#;
///     if let Ok(get)=fn_str::get_mid_str("HREF=\"","\"",s){
///         assert_eq!(r#"http://web.stanford.edu/class/cs97si/"#.to_string(),get);
///     }
///     if let Ok(get)=fn_str::get_mid_str("\">","</A>",s){
///         assert_eq!(r#"CS 97SI: Introduction to Programming Contests"#.to_string(),get);
///     }
/// ```
pub fn get_mid_str<S:AsRef<str>>(start:S,end:S,s:S)->Result<String>
{
    let mut lead:usize;
    let trail:usize;
    match s.as_ref().find(start.as_ref()){
        Some(first) => {
            lead=first;
            let now=first+start.as_ref().len();
            let ss=&s.as_ref()[now..];
            match ss.find(end.as_ref()) {
                Some(last)=>{
                    trail =now+last
                },
                None=>{return Err("No find end substring.".into());},
            }
        },
        None => {return Err("No find start substring.".into())},
    }
    if lead > trail || trail >s.as_ref().len(){
        return Err("Substring?".into())
    }
    lead += start.as_ref().len();
    let slice=&s.as_ref()[lead..trail];
    Ok(slice.to_string())
}

///Command get middle subject str.
///Example:
/// ```rust
/// use jay_lib::fns::fn_str;
///    let s=r#"<DT><A HREF="https://blog.csdn.net/sdu_hao/article/details/95958293?utm_medium=distribute.pc_relevant.none-task-blog-BlogCommendFromMachineLearnPai2-3.control&depth_1-utm_source=distribute.pc_relevant.none-task-blog-BlogCommendFromMachineLearnPai2-3.control" ADD_DATE="1612008593" ICON="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAIAAACQkWg2AAAB/klEQVQ4jU2SPWtUYRCFz5n3bvbubhISoyJiJxEsEkglghZJZUTUJkhAUoja2WihgohFQPAXaGVhk8rGRkQrURDEFOL3B2idkDXJ7t3Nve8ci7ubZOozZ2aeMyzmptAvSSBpBgkxgkQIkOQOkgCAZLeaJEi1NkBDWgeEzXUkFVZTuIPcaSjVkqPb5bEZTp/H4aPMc//0Hs+X9Psb0hrlILm9kuRw5+XbNjsPuX//yNEx239Inbbfu6IvH5jWISU9fzNlmV26FWbn/d0rf7SI5oosaPosF25gZC9iLH1ZzE0JRLeN8YnwYEl/fsSb8+xkTOvyiK0O9h3ERhMSAZAGgKTy3E6eppmePcHmOhtDgmjG2iDXVtjnAcAAyCNrDYxPSNLXZVZTxYIlZTmSBKSkciUrb0BSwfAe5DnW11DmUE7u6bhN3/opRHTbSALSGgSaARDAEOCuYmu72QAwJGq39OszLXDqhP6tKjrc6VEbTdUaHDugIufODRAt6OVTxcIWrtuZi6imCokGUkwet7sPbfExBqpy72Etc0C7xVMX7OodVCroZGquojHIoRECvvzG71+DnLRdSQPIWhiftJlzODLJ4VF1Mvz9qeXXevsCRcEQ+hN2Ps/UaaPIUWswJJKjm6EoWB8syZLsPV8PgiLTGlhHjIoFSaR10OQRpSPwH79sFlWOAVADAAAAAElFTkSuQmCC">自然语言处理 | (30) 文本相似度计算与文本匹配问题_sdu_hao的博客-CSDN博客_文本相似度匹配</A>"#;
///     if let Ok(after_del)=fn_str::del_mid_str(" ICON=\"","\">",s){
///         assert_eq!(r#"<DT><A HREF="https://blog.csdn.net/sdu_hao/article/details/95958293?utm_medium=distribute.pc_relevant.none-task-blog-BlogCommendFromMachineLearnPai2-3.control&depth_1-utm_source=distribute.pc_relevant.none-task-blog-BlogCommendFromMachineLearnPai2-3.control" ADD_DATE="1612008593"自然语言处理 | (30) 文本相似度计算与文本匹配问题_sdu_hao的博客-CSDN博客_文本相似度匹配</A>"#.to_string(),after_del);
///     }
///     if let Ok(after_del)=fn_str::del_mid_str(" ADD_DATE=\"","\">",s){
///         assert_eq!(r#"<DT><A HREF="https://blog.csdn.net/sdu_hao/article/details/95958293?utm_medium=distribute.pc_relevant.none-task-blog-BlogCommendFromMachineLearnPai2-3.control&depth_1-utm_source=distribute.pc_relevant.none-task-blog-BlogCommendFromMachineLearnPai2-3.control"自然语言处理 | (30) 文本相似度计算与文本匹配问题_sdu_hao的博客-CSDN博客_文本相似度匹配</A>"#.to_string(),after_del);
///     }
///         let s=r#"<DT><A HREF="https://www.jianshu.com/p/c7421770d0d1" ADD_DATE="1612009180" ICON="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAACz0lEQVQ4jV2ST2hcZRTFf+f7vvfezOuk0cTWaYtVUEGyEhV0oaBQ0uJCpV1lUwuCBSMoLQiiIoJ/KNiCi1hdxC4UBP/gUtCNiOBG3XRRin9qaJPUUs1Mp0ln3nvfd11MI8WzvNx77uGco5VDe/bnLj9Rp3RbAoiNzHlJYhOWEmDIeQPMSWTS+SpVR4JZWADrRkjCpHZH1COsqsbX3qMsB+ewaxsSRjQsK/LbjbDgJLpDw9Kg7/KZ+7Tj5JcW7rgbTWwldHdhdc3Ui28wffQtlOfmpm+V39Z1w2Qm6Gr50L5ksZGfmLTpl9/FYiM/vZ1meYnqz1+xqwO27N1Pfe4s2Z330Px2xuqV81z5/CNZXZsjRchypl46RnNpVZdeOUz1x1mGp39i8NUntB+ZZe3k2/x9/DVsY52r33+j3uJxLCXkHbpwcNbULnFlh9a9D9J+6FHWv/uaiX0HMAlCIP1zGdcuIcuRHP1PP2D4y4+4dkmQc9jGOnE0pL5wjrL1OMpy+p8t4soOFhsUAmk0RHJMvfA6bnIKixGTCCaRRtfo7HmSyYPPs/rcASbnDtOZfYo46CM53JYO1e9n6J16b5yOJSSQGUESMgMYxxUjYftOLEXW3n8HUsPN86+S3TWDihZyjv8qIhEwAwkzGxfGedJwg/yWGW565ghgZDt30/y1AhJcf7aJMB6CpOsLCdcqaZaX6H14DARTR99EeQEpYjdymBEwM4QAI0YhEQd9wq7dbJ17FuUFKtr0P14gXemPlbtgmAnJwpjIkA+4PEd5QW/xBK7dRnkLgDTo4bftoPPEHC5kpMEaOD9WIJCct2btstZ/+BaqkVHXSk0FqTf2p6kpH7uf8oGHrf/FKQ1P/ywVLZOZtPz03tUi+O6wiYm6ElkmyfF/WNOAgKoyipa1suBGTbropDiPseQlVLRMyOxGpzcjDgE5byo75r0HY0lq5v8FS/1LVvsjW/AAAAAASUVORK5CYII=">文本的匹配 - 简书</A>"#;
///     if let Ok(after_del)=fn_str::del_mid_str(" ADD_DATE=\"","\">",s) {
///         assert_eq!(r#"<DT><A HREF="https://www.jianshu.com/p/c7421770d0d1"文本的匹配 - 简书</A>"#.to_string(),after_del);
///     }
/// ```
pub fn del_mid_str<S>(start:S,end:S,s:S)->Result<String>
    where
        S:AsRef<str>,
{
    let lead:usize;
    let trail:usize;
    match s.as_ref().find(start.as_ref()){
        Some(first) => {
            lead=first;
            let now=first+start.as_ref().len();
            let ss=&s.as_ref()[now..];
            match ss.find(end.as_ref()) {
                Some(last)=>{
                    trail =now+last+end.as_ref().len()
                },
                None=>{return Err("No find end substring.".into())},
            }
        },
        None => {return Err("No find start substring.".into())},
    }
    if lead > trail || trail >s.as_ref().len(){
        return Err("Substring?".into())
    }
    let front=&s.as_ref()[0..lead];
    let back=&s.as_ref()[trail..];
    let slice = front.to_owned()+back;
    Ok(slice)
}
///Check the string is or not English
/// But, if it is a phrase.
/// What to do?
/// ```rust
///     use jay_lib::fns::fn_str;
///     assert_eq!(false,fn_str::check_word(""));
///     assert_eq!(false,fn_str::check_word("123 eng"));
///     assert_eq!(false,fn_str::check_word("what is it"));
///     assert_eq!(true,fn_str::check_word("hello"));
///     assert_eq!(true,fn_str::check_word("It's"));
///```
pub fn check_word<S: ToString>(s: S) -> bool{
    if s.to_string().is_empty() { return false; };
    let s: String = s.to_string().chars().filter(|c| !c.is_alphabetic()&&*c!='\'').collect();
    s.is_empty()
}
///Check the string has Chinese.
///```rust
///     use jay_lib::fns::fn_str;
///     assert_eq!(false,fn_str::check_zh("".to_string()));
///     assert_eq!(true,fn_str::check_zh("中".to_string()));
///     assert_eq!(true,fn_str::check_zh("go 中".to_string()));
///     assert_eq!(true,fn_str::check_zh("裏".to_string()));
///```
pub fn check_zh(s: String) -> bool {
    let zh: String = s.chars().into_iter().filter(|x| x > &'\u{4E00}' && x < &'\u{9FFF}').collect();
    !zh.is_empty()
}
///0.1.2.3.4.5.6.7.8.9
///```rust
///     use jay_lib::fns::fn_str;
///     assert_eq!(false,fn_str::is_number("".to_string()));
///     assert_eq!(true,fn_str::is_number("1".to_string()));
///     assert_eq!(false,fn_str::is_number("1 1".to_string()));
///     assert_eq!(true,fn_str::is_number("789".to_string()));
/// ```
pub fn is_number(s: String) -> bool {
    if s.is_empty() { return false; };
    let number:String=s.chars().filter(|c|c.is_digit(10)).collect();
    s.len()==number.len()
}
