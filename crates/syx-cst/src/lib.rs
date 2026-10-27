//! ADR-142 / M1-E04: lossless CST skeleton — trivia-preserving subset for `func main()`.
//! W683 / ADR-1027: body may hold a single `print(<int>)` stmt (C-06 first slice).
//! W790 / ADR-1134: body may hold 0..N `print(<int|ident|string>)` stmts (comment-preserving deepen).
//! W791 / ADR-1135: body may interleave `let <ident> = <simple>` with print stmts.
//! W792 / ADR-1136: body may interleave `return <simple>` with print / let stmts.
//! W793 / ADR-1137: `is_simple_value` includes `Double` (print/let/return RHS).
//! W794 / ADR-1138: `is_simple_value` includes `Bool` (print/let/return RHS).
//! W795 / ADR-1139: `is_simple_value` includes `Nil` (print/let/return RHS).
//! W796 / ADR-1140: body may interleave `var <ident> = <simple>` with print / let / return.
//! W797 / ADR-1141: body may hold bare `return` / `return;` interleaved with print / let / var / return-simple.
//! W798 / ADR-1142: file may hold 1..N consecutive zero-arg `func name() { … }` (each body as above).
//! W799 / ADR-1143: let/var may hold optional `: Ident` typed bind before `=`.
//! W800 / ADR-1144: typed bind may hold optional `: Ident?` (Optional Named).
//! W801 / ADR-1145: typed bind may hold optional `: &Ident` (immutable Reference Named).
//! W802 / ADR-1146: typed bind may hold optional `: &mut Ident` (mutable Reference Named).
//! W803 / ADR-1147: typed bind may hold optional `: &'a Ident` / `: &'a mut Ident` (lifetime Reference).
//! W804 / ADR-1148: typed bind may hold optional `: (Ident…)[-> Ident]` / `: ()` (Function TypeExpr).
//! W805 / ADR-1149: Function TypeExpr params may be Amp Reference family (same as bind).
//! W806 / ADR-1150: Function TypeExpr params may be Optional Named (`Ident?`).
//! W807 / ADR-1151: Function TypeExpr return may be Amp Reference family.
//! W808 / ADR-1152: Function TypeExpr return may be Optional Named (`Ident?`).
//! W809 / ADR-1153: Function TypeExpr params may be nested Function TypeExpr.
//! W810 / ADR-1154: Function TypeExpr return may be nested Function TypeExpr.
//! W811 / ADR-1155: Amp Reference Named may hold trailing `?` (Optional of Reference).
//! W812 / ADR-1156: Named may hold single-arg generic `Ident < Ident >` [`?`].
//! W813 / ADR-1157: Named generic arg may be nested Named (`Foo<Bar<Int>>` [`?`]).
//! W814 / ADR-1158: Named generic may hold multiple args (`Foo<A, B>` [`?`]).
//! W815 / ADR-1159: module top may interleave simple stmts with 1..N zero-arg funcs.
//! W816 / ADR-1160: func may hold bare Ident params (`func f(x, y) { … }`).
//! W817 / ADR-1161: func params may be typed Named Ident (`func f(x: Int, y) { … }`).
//! W818 / ADR-1162: func params may be Optional Named (`func f(x: Int?, y) { … }`).
//! W819 / ADR-1163: func params may be Amp Named (`func f(x: &Int, y) { … }`).
//! W820 / ADR-1164: func params may be mut Amp Named (`func f(x: &mut Int, y) { … }`).
//! W821 / ADR-1165: func params may be lifetime Amp Named (`func f(x: &'a Int, y) { … }`).
//! W822 / ADR-1166: func params may be Amp family with trailing `?` (`func f(x: &Int?, y) { … }`).
//! W823 / ADR-1167: func params may be single-arg Named generic (`func f(x: Foo<Int>, y) { … }`).
//! W824 / ADR-1168: func params may be nested single-arg Named generic (`func f(x: Foo<Bar<Int>>, y) { … }`).
//! W825 / ADR-1169: func params may be multi-arg Named generic (`func f(x: Foo<A, B>, y) { … }`).
//! W826 / ADR-1170: func may hold optional return type (`func f() -> Int { … }` / `-> Int?`).
//! W827 / ADR-1171: func return may be Amp Named (`func f() -> &Int { … }`).
//! W828 / ADR-1172: func return may be mut Amp Named (`func f() -> &mut Int { … }`).
//! W829 / ADR-1173: func return may be lifetime Amp Named (`func f() -> &'a Int { … }` / `-> &'a mut Int`).
//! W830 / ADR-1174: func return may hold Amp-family trailing `?` (`func f() -> &Int? { … }` / `-> &'a mut Int?`).
//! W831 / ADR-1175: func return may be single-arg Named generic (`func f() -> Foo<Int> { … }` / `-> Bar<String>?`).
//! W832 / ADR-1176: func return may be nested single-arg Named generic (`func f() -> Foo<Bar<Int>> { … }` / `-> Box<List<String>>?`).
//! W833 / ADR-1177: bundle — func return multi-arg Named generic / Function TypeExpr return / Amp-as-generic-arg (`-> Foo<A, B>` / `-> (Int) -> Bool` / `Foo<&Int>`).
//! W834 / ADR-1178: bundle — Function-as-generic-arg + call-simple as print arg / let|var|return RHS.
//! W835 / ADR-1179: bundle — while-simple (ident|bool|call-simple cond; body = print|let|var|return; no nested while).
//! W836 / ADR-1180: bundle — nested while + compare cond (`<`/`>`/`<=`/`>=`) + deeper mixed; still no effect / nested-call / brace-less / logic cond.
//! W837 / ADR-1181: bundle — func shell optional `requires Pure|IO` (effect); still no nested-call / throws / where.
//! W838 / ADR-1182: bundle — nested-call-as-arg depth≤1 in call-simple (print/RHS/while cond); still no throws / where / depth>1.
//! W839 / ADR-1183: bundle — func shell optional `throws` (before requires); still no type_params / where / depth>1.
//! W840 / ADR-1184: bundle — func shell optional type_params `<Ident(, Ident)*>` before `(`; still no where / depth>1.
//! W841 / ADR-1185: bundle — func shell optional `where Ident: Ident(, …)*` after requires; still no depth>1 / brace-less / logic cond.
//! W842 / ADR-1186: bundle — nested-call-as-arg depth≤2 in call-simple (print/RHS/while cond); still no depth>2 / brace-less / logic cond.
//! W843 / ADR-1187: bundle — brace-less while (single print|let|var|return body); still no logic cond / depth>2.
//! W844 / ADR-1188: bundle — while-cond logic `simple &&|or|| simple` (single binary); still no chained / depth>2.
//! W845 / ADR-1189: bundle — while-cond logic same-op chain + compare×logic operands; still no mixed-ops / call×logic / depth>2.
//! W846 / ADR-1190: bundle — while-cond mixed-ops left-assoc + call×logic operands; still no unary ! / paren / depth>2.
//! W847 / ADR-1191: bundle — while-cond unary `!` (bare + unary×logic operands); still no paren / `!!` / depth>2.
//! W848 / ADR-1192: bundle — while-cond paren group depth=1 (+ paren×unary/call); still no Neg `-` / nested paren / `!!`.
//! W849 / ADR-1193: bundle — while-cond unary Neg `-` (bare + Neg×logic/call); still no `!!` / `--` / nested paren.
//! W850 / ADR-1194: bundle — while-cond stacked `!!` (Bang depth≤2); still no `!!!` / `--` / nested paren.
//! W851 / ADR-1195: bundle — while-cond nested paren depth≤2 (`((x))`); still no `(((…)))` / `!!!` / `--`.
//! W852 / ADR-1196: bundle — while-cond stacked `!!!` (Bang depth≤3); still no `!!!!` / `--` / paren depth>2.
//! W853 / ADR-1197: bundle — while-cond stacked `--` (Neg depth≤2); still no `---` / `!!!!` / paren depth>2.
//! W854 / ADR-1198: bundle — while-cond stacked `---` (Neg depth≤3); still no `----` / `!!!!` / paren depth>2.
//! W855 / ADR-1199: bundle — while-cond stacked `----` (Neg depth≤4); still no `-----` / `!!!!` / paren depth>2.
//! W856 / ADR-1200: bundle — while-cond stacked `-----` (Neg depth≤5); still no `------` / `!!!!` / paren depth>2.
//! W857 / ADR-1201: bundle — while-cond stacked `------` (Neg depth≤6); still no `-------` / `!!!!` / paren depth>2.
//! W858 / ADR-1202: bundle — while-cond stacked `-------` (Neg depth≤7); still no `--------` / `!!!!` / paren depth>2.
//! W859 / ADR-1203: bundle — while-cond stacked `--------` (Neg depth≤8); still no `---------` / `!!!!` / paren depth>2.
//! W860 / ADR-1204: bundle — while-cond stacked `---------` (Neg depth≤9); still no `----------` / `!!!!` / paren depth>2.
//! W861 / ADR-1205: bundle — while-cond stacked `----------` (Neg depth≤10); still no `-----------` / `!!!!` / paren depth>2.
//! W862 / ADR-1206: bundle — while-cond stacked `-----------` (Neg depth≤11); still no `------------` / `!!!!` / paren depth>2.
//! W863 / ADR-1207: bundle — while-cond stacked `------------` (Neg depth≤12); still no `-------------` / `!!!!` / paren depth>2.
//! W864 / ADR-1208: bundle — while-cond stacked `-------------` (Neg depth≤13); still no `--------------` / `!!!!` / paren depth>2.
//! W865 / ADR-1209: bundle — while-cond stacked `--------------` (Neg depth≤14); still no `---------------` / `!!!!` / paren depth>2.
//! W866 / ADR-1210: bundle — while-cond stacked `---------------` (Neg depth≤15); still no `----------------` / `!!!!` / paren depth>2.
//! W867 / ADR-1211: bundle — while-cond stacked `----------------` (Neg depth≤16); still no `-----------------` / `!!!!` / paren depth>2.
//! W868 / ADR-1212: bundle — while-cond stacked `-----------------` (Neg depth≤17); still no `------------------` / `!!!!` / paren depth>2.
//! W869 / ADR-1213: bundle — while-cond stacked `------------------` (Neg depth≤18); still no `-------------------` / `!!!!` / paren depth>2.
//! W870 / ADR-1214: bundle — Neg cap frozen at ≤18; while-cond stacked `!!!!` (Bang depth≤4); still no `!!!!!` / `-------------------` / paren depth>2.
//! W871 / ADR-1215: bundle — paren nest cap frozen at ≤3 (`(((x)))`); Neg≤18 and Bang≤4 still frozen; still no `((((…))))` / `!!!!!` / `-------------------`.

use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_lexer::{Lexer, Token, TokenKind};

/// Leading/trailing trivia attached conceptually to a token stream position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trivia {
    pub text: String,
}

/// Green token node (lexeme preserved for round-trip).
#[derive(Debug, Clone, PartialEq)]
pub struct GreenToken {
    pub kind: TokenKind,
    pub lexeme: String,
}

/// Minimal green tree: ordered tokens including trivia (comments / whitespace / newlines).
#[derive(Debug, Clone, PartialEq)]
pub struct GreenNode {
    pub tokens: Vec<GreenToken>,
}

/// CST root for the W6 vertical slice.
#[derive(Debug, Clone, PartialEq)]
pub struct CstRoot {
    pub source_name: String,
    pub green: GreenNode,
}

fn is_trivia(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Newline
            | TokenKind::LineComment
            | TokenKind::BlockComment
            | TokenKind::Whitespace
            | TokenKind::Eof
    )
}

fn significant(tokens: &[Token]) -> Vec<&Token> {
    tokens
        .iter()
        .filter(|t| !is_trivia(&t.kind))
        .collect()
}

/// Parse a **subset** CST: optional trivia + 1..N `func <name>([Ident[: Ident[<Named(, Named)*>][?]| &Ident[?]| &mut Ident[?]| &'a Ident[?]| &'a mut Ident[?]…](, …)*]) [-> Ident[<NestedNamed>][?]| &Ident[?]| &mut Ident[?]| &'a Ident[?]| &'a mut Ident[?]] { … }`
/// interleaved with top-level simple stmts (print / let / var / return) (W683–W832);
/// each body trivia-only or 0..N of the same simple stmts;
/// let/var may include optional `: Ident[<Named(, Named)*>][?]` / `: &Ident[?]` / `: &mut Ident[?]` /
/// `: &'a Ident[?]` / `: &'a mut Ident[?]` /
/// `: (Ident[<Named(, Named)*>][?]|Amp[?]…|nested Function)[-> Ident[<Named(, Named)*>][?]|Amp[?]…|nested Function]` / `: ()` (W799–W814).
pub fn parse_cst_subset(source: &SourceFile) -> SyxResult<CstRoot> {
    let tokens = Lexer::new(source.clone()).scan_all()?;
    validate_func_main_subset(&tokens)?;
    let green = GreenNode {
        tokens: tokens
            .into_iter()
            .filter(|t| t.kind != TokenKind::Eof)
            .map(|t| GreenToken {
                kind: t.kind,
                lexeme: t.lexeme.to_string(),
            })
            .collect(),
    };
    Ok(CstRoot {
        source_name: source.name.clone(),
        green,
    })
}

fn is_simple_value(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Integer(_, _)
            | TokenKind::Double(_, _)
            | TokenKind::Bool(_)
            | TokenKind::Nil
            | TokenKind::Identifier(_)
            | TokenKind::String(_)
            // W900: `self` as call/return operand (builtins extension bodies).
            | TokenKind::SelfKw
    )
}

/// Optional external-label prefix `Ident :` before a call arg (W900).
fn take_call_arg_label(tokens: &[&Token], j: usize) -> usize {
    if j + 1 < tokens.len()
        && matches!(tokens[j].kind, TokenKind::Identifier(_))
        && tokens[j + 1].kind == TokenKind::Colon
    {
        j + 2
    } else {
        j
    }
}

/// Consume one call arg at `tokens[j]`: optional `Ident :` label, then nested call-simple
/// (if `depth > 0`), member access, or bare simple value. At `depth == 0`, `Ident (` is rejected.
/// Returns index after the arg, or `None`.
fn take_call_arg(tokens: &[&Token], j: usize, depth: usize) -> Option<usize> {
    let j = take_call_arg_label(tokens, j);
    if depth > 0 {
        if let Some(end) = take_call_simple_at(tokens, j, depth - 1) {
            return Some(end);
        }
    }
    // W900: `self.field` / `ident.field` as call arg (file_handle).
    if let Some(end) = take_member_access_simple(tokens, j) {
        return Some(end);
    }
    if let Some(end) = take_self_member_access(tokens, j) {
        return Some(end);
    }
    if j >= tokens.len() || !is_simple_value(&tokens[j].kind) {
        return None;
    }
    // Ident followed by `(` / `.` is a call/member — only allowed via recursion above.
    if matches!(tokens[j].kind, TokenKind::Identifier(_))
        && j + 1 < tokens.len()
        && matches!(tokens[j + 1].kind, TokenKind::LParen | TokenKind::Dot)
    {
        return None;
    }
    Some(j + 1)
}

/// W900: `self (. Ident)+` member access (call arg / return RHS).
fn take_self_member_access(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || tokens[i].kind != TokenKind::SelfKw {
        return None;
    }
    let mut j = i + 1;
    let mut dots = 0usize;
    while j + 1 < tokens.len()
        && tokens[j].kind == TokenKind::Dot
        && matches!(tokens[j + 1].kind, TokenKind::Identifier(_))
    {
        j += 2;
        dots += 1;
    }
    if dots == 0 {
        return None;
    }
    if j < tokens.len() && tokens[j].kind == TokenKind::LParen {
        return None;
    }
    Some(j)
}

/// Consume call-simple at `tokens[i]` with nesting budget `depth`.
/// `Ident (. Ident)* ( [arg (, arg)*] )` where arg = optional `Ident :` + call-simple (depth-1) | simple.
/// Entry depth=2 allows `foo(bar(baz()))` but rejects `foo(bar(baz(qux())))`.
/// W900: member/Type method calls (`Float32.min(a, b)` / `items.contains(item)`).
fn take_call_simple_at(tokens: &[&Token], i: usize, depth: usize) -> Option<usize> {
    if i >= tokens.len() || !matches!(tokens[i].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut j = i + 1;
    // Optional member chain before `(`.
    while j + 1 < tokens.len()
        && tokens[j].kind == TokenKind::Dot
        && matches!(tokens[j + 1].kind, TokenKind::Identifier(_))
    {
        j += 2;
    }
    if j >= tokens.len() || tokens[j].kind != TokenKind::LParen {
        return None;
    }
    j += 1;
    if j < tokens.len() && tokens[j].kind == TokenKind::RParen {
        return Some(j + 1);
    }
    j = take_call_arg(tokens, j, depth)?;
    while j < tokens.len() && tokens[j].kind == TokenKind::Comma {
        j += 1;
        j = take_call_arg(tokens, j, depth)?;
    }
    if j >= tokens.len() || tokens[j].kind != TokenKind::RParen {
        return None;
    }
    Some(j + 1)
}

/// Consume call-simple at `tokens[i]`: `Ident (. Ident)* ( [arg…] )` with nested-call depth≤2 (W842/W900).
/// Returns index after `)`, or `None`.
fn take_call_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    take_call_simple_at(tokens, i, 2)
}

/// W900: bare member access `Ident (. Ident)+` (not a call). Used as return RHS (`items.length`).
fn take_member_access_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || !matches!(tokens[i].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut j = i + 1;
    let mut dots = 0usize;
    while j + 1 < tokens.len()
        && tokens[j].kind == TokenKind::Dot
        && matches!(tokens[j + 1].kind, TokenKind::Identifier(_))
    {
        j += 2;
        dots += 1;
    }
    if dots == 0 {
        return None;
    }
    // Call form handled by take_call_simple.
    if j < tokens.len() && tokens[j].kind == TokenKind::LParen {
        return None;
    }
    Some(j)
}

/// W900: `@builtin("…")` attribute prefix before a decl.
/// W901: also bare `@Ident` / `@Ident(...)` (e.g. `@Codable`).
fn take_attr_prefix(sig: &[&Token], i: usize) -> Option<usize> {
    if i + 1 >= sig.len() || sig[i].kind != TokenKind::At {
        return None;
    }
    if !matches!(sig[i + 1].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut j = i + 2;
    // Optional `( args )` — string or ident args, commas OK (`@builtin("x")` / `@freestanding(expression)`).
    if j < sig.len() && sig[j].kind == TokenKind::LParen {
        j += 1;
        if j < sig.len() && sig[j].kind != TokenKind::RParen {
            loop {
                if j >= sig.len() {
                    return None;
                }
                match &sig[j].kind {
                    TokenKind::String(_) | TokenKind::Identifier(_) => j += 1,
                    _ => return None,
                }
                if j < sig.len() && sig[j].kind == TokenKind::Comma {
                    j += 1;
                    continue;
                }
                break;
            }
        }
        if j >= sig.len() || sig[j].kind != TokenKind::RParen {
            return None;
        }
        j += 1;
    }
    Some(j)
}

/// W901: `import Ident` top-level.
fn take_import_shell(sig: &[&Token], i: usize) -> Option<usize> {
    if i + 1 >= sig.len() || sig[i].kind != TokenKind::Import {
        return None;
    }
    if !matches!(sig[i + 1].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut j = i + 2;
    if j < sig.len() && sig[j].kind == TokenKind::Semicolon {
        j += 1;
    }
    Some(j)
}

/// One print call: `print` `(` simple|call-simple `)` [`;`].
/// Returns the number of significant tokens consumed, or `None` if not a match.
fn take_print_simple_call(body: &[&Token]) -> Option<usize> {
    if body.len() < 4 {
        return None;
    }
    match &body[0].kind {
        TokenKind::Identifier(name) if name.as_str() == "print" => {}
        _ => return None,
    }
    if body[1].kind != TokenKind::LParen {
        return None;
    }
    // W834: prefer call-simple arg (`print(f())` / `print(g(1))`).
    let after_arg = if let Some(end) = take_call_simple(body, 2) {
        end
    } else if is_simple_value(&body[2].kind) {
        // Bare Ident (not followed by `(`) / literal.
        if matches!(body[2].kind, TokenKind::Identifier(_))
            && body.len() > 3
            && body[3].kind == TokenKind::LParen
        {
            return None;
        }
        3
    } else {
        return None;
    };
    if after_arg >= body.len() || body[after_arg].kind != TokenKind::RParen {
        return None;
    }
    let end = after_arg + 1;
    if body.len() > end && body[end].kind == TokenKind::Semicolon {
        Some(end + 1)
    } else {
        Some(end)
    }
}

/// Consume Amp Reference Named starting at `tokens[i]` (`&` already at `i`).
/// Returns index after the type Ident [`?`] (W811 optional `?`), or `None`.
fn take_amp_reference_named(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || tokens[i].kind != TokenKind::Amp {
        return None;
    }
    if i + 1 >= tokens.len() {
        return None;
    }
    let mut j = i + 1;
    if matches!(tokens[j].kind, TokenKind::Lifetime(_)) {
        j += 1;
        if j >= tokens.len() {
            return None;
        }
    }
    if matches!(&tokens[j].kind, TokenKind::Identifier(name) if name.as_str() == "mut") {
        j += 1;
        if j >= tokens.len() {
            return None;
        }
    }
    if !matches!(tokens[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    // W811: optional postfix `?` after Amp Reference Named (`&T?`).
    if j < tokens.len() && tokens[j].kind == TokenKind::Question {
        j += 1;
    }
    Some(j)
}

/// Consume Named type starting at `tokens[i]`: `Ident` [`Less` Named (`,` Named)* `Greater`] [`?`].
/// W812: optional generic; W813: arg may nest Named; W814: multi-arg via commas;
/// W800/W806/W808: optional trailing `?`.
/// Returns index after the type, or `None`.
fn take_named_type(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || !matches!(tokens[i].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut j = i + 1;
    // W812–W814: optional `< Named (, Named)* >`.
    if j < tokens.len() && tokens[j].kind == TokenKind::Less {
        j += 1;
        j = take_named_type(tokens, j)?;
        while j < tokens.len() && tokens[j].kind == TokenKind::Comma {
            j += 1;
            j = take_named_type(tokens, j)?;
        }
        if j >= tokens.len() || tokens[j].kind != TokenKind::Greater {
            return None;
        }
        j += 1;
    }
    // Optional postfix `?` after Named / Named-generic.
    if j < tokens.len() && tokens[j].kind == TokenKind::Question {
        j += 1;
    }
    Some(j)
}

/// Consume one Function TypeExpr param: Named Ident[`<Ident>`][`?`] (W806/W812), Amp Reference (W805),
/// or nested Function TypeExpr (W809). Returns index after the param type, or `None`.
fn take_function_param_type(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() {
        return None;
    }
    if tokens[i].kind == TokenKind::Amp {
        return take_amp_reference_named(tokens, i);
    }
    if tokens[i].kind == TokenKind::LParen {
        // W809: nested Function TypeExpr as param.
        return take_function_type_expr(tokens, i);
    }
    take_named_type(tokens, i)
}

/// Consume Function TypeExpr starting at `tokens[i]` (`(` already at `i`).
/// `( param (`,` param)* )?` [ `->` Ident[?]|Amp|nested Function ];
/// param may recurse (W809); return may recurse (W810). Returns index after the type, or `None`.
fn take_function_type_expr(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || tokens[i].kind != TokenKind::LParen {
        return None;
    }
    let mut j = i + 1;
    if j >= tokens.len() {
        return None;
    }
    if tokens[j].kind != TokenKind::RParen {
        j = take_function_param_type(tokens, j)?;
        while j < tokens.len() && tokens[j].kind == TokenKind::Comma {
            j += 1;
            j = take_function_param_type(tokens, j)?;
        }
        if j >= tokens.len() || tokens[j].kind != TokenKind::RParen {
            return None;
        }
    }
    j += 1; // consume RParen
    // optional `-> Ident[<Ident>][?]` | Amp[?] Reference | nested Function (W808–W812)
    if j < tokens.len() && tokens[j].kind == TokenKind::Arrow {
        j += 1;
        if j >= tokens.len() {
            return None;
        }
        if tokens[j].kind == TokenKind::Amp {
            j = take_amp_reference_named(tokens, j)?;
        } else if tokens[j].kind == TokenKind::LParen {
            // W810: nested Function TypeExpr as return.
            j = take_function_type_expr(tokens, j)?;
        } else {
            // W808/W812: Named return Ident[`<Ident>`][`?`].
            j = take_named_type(tokens, j)?;
        }
    }
    Some(j)
}

/// One let/var bind: `let|var` `Ident` [`:` type] `=` simple [`;`].
/// Optional `: Ident` is W799 Named; trailing `?` is W800 Optional;
/// `: Ident < Ident >` is W812 single-arg generic Named;
/// `: &Ident` is W801 Reference; `: &mut Ident` is W802 mutable Reference;
/// `: &'a Ident` / `: &'a mut Ident` is W803 lifetime Reference;
/// `: (Ident|&Ident|…)[-> Ident]` / `: ()` is W804 Function TypeExpr;
/// Function params may be Amp Reference (W805), Optional Named (W806), nested Function (W809), Amp+`?` (W811), or Named generic (W812);
/// Function return may be Amp Reference (W807), Optional Named (W808), nested Function (W810), Amp+`?` (W811), or Named generic (W812).
fn take_simple_bind(body: &[&Token], kw: TokenKind) -> Option<usize> {
    if body.len() < 4 || body[0].kind != kw {
        return None;
    }
    if !matches!(body[1].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut i = 2usize;
    // Optional `: Ident[<Ident>][?]` | `: & [Lifetime] [mut] Ident` | `: ( param… ) [-> ret]`
    if i < body.len() && body[i].kind == TokenKind::Colon {
        i += 1;
        if i >= body.len() {
            return None;
        }
        if body[i].kind == TokenKind::Amp {
            // W801–W803: Reference Named
            i = take_amp_reference_named(body, i)?;
        } else if body[i].kind == TokenKind::LParen {
            // W804–W812: Function TypeExpr (params/return may nest Function; Amp/Named may hold `?`/generic)
            i = take_function_type_expr(body, i)?;
        } else {
            // W799–W800 / W812: Named Ident[`<Ident>`][`?`]
            i = take_named_type(body, i)?;
        }
    }
    if i >= body.len() || body[i].kind != TokenKind::Assign {
        return None;
    }
    i += 1;
    // W834/W900: RHS may be call-simple, member access, or simple value.
    let after_rhs = if let Some(end) = take_call_simple(body, i) {
        end
    } else if let Some(end) = take_member_access_simple(body, i) {
        end
    } else if let Some(end) = take_self_member_access(body, i) {
        end
    } else if i < body.len() && is_simple_value(&body[i].kind) {
        if matches!(body[i].kind, TokenKind::Identifier(_))
            && i + 1 < body.len()
            && matches!(body[i + 1].kind, TokenKind::LParen | TokenKind::Dot)
        {
            return None;
        }
        i + 1
    } else {
        return None;
    };
    if body.len() > after_rhs && body[after_rhs].kind == TokenKind::Semicolon {
        Some(after_rhs + 1)
    } else {
        Some(after_rhs)
    }
}

/// One let bind: `let` `Ident` [`:` type] `=` simple|call-simple [`;`].
fn take_let_simple_bind(body: &[&Token]) -> Option<usize> {
    take_simple_bind(body, TokenKind::Let)
}

/// One var bind: `var` `Ident` [`:` type] `=` simple|call-simple [`;`].
fn take_var_simple_bind(body: &[&Token]) -> Option<usize> {
    take_simple_bind(body, TokenKind::Var)
}

/// One return: `return` [simple|call-simple] [`;`].
/// Bare `return` / `return;` and `return <simple|call>` are accepted (W797 / W834).
/// W900: `throw <call-simple|member|simple>` [`;`].
fn take_throw_simple(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || body[0].kind != TokenKind::Throw {
        return None;
    }
    let end = if let Some(end) = take_call_simple(body, 1) {
        end
    } else if let Some(end) = take_member_access_simple(body, 1) {
        end
    } else if let Some(end) = take_self_member_access(body, 1) {
        end
    } else if body.len() >= 2 && is_simple_value(&body[1].kind) {
        if matches!(body[1].kind, TokenKind::Identifier(_))
            && body.len() >= 3
            && matches!(body[2].kind, TokenKind::LParen | TokenKind::Dot)
        {
            return None;
        }
        2
    } else {
        return None;
    };
    if body.len() > end && body[end].kind == TokenKind::Semicolon {
        Some(end + 1)
    } else {
        Some(end)
    }
}

/// W900: `try { stmts } catch ( Ident ) { stmts }` — bodies must be simple-stmts.
fn take_try_catch_simple(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || body[0].kind != TokenKind::Try {
        return None;
    }
    if body.len() < 2 || body[1].kind != TokenKind::LBrace {
        return None;
    }
    // Find matching RBrace for try body.
    let mut depth = 1usize;
    let mut k = 2usize;
    while k < body.len() {
        match body[k].kind {
            TokenKind::LBrace => depth += 1,
            TokenKind::RBrace => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        k += 1;
    }
    if k >= body.len() || body[k].kind != TokenKind::RBrace {
        return None;
    }
    let try_body = &body[2..k];
    if !is_simple_stmts_body(try_body) {
        return None;
    }
    let mut j = k + 1;
    if j >= body.len() || body[j].kind != TokenKind::Catch {
        return None;
    }
    j += 1;
    if j >= body.len() || body[j].kind != TokenKind::LParen {
        return None;
    }
    j += 1;
    if j >= body.len() || !matches!(body[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    if j >= body.len() || body[j].kind != TokenKind::RParen {
        return None;
    }
    j += 1;
    if j >= body.len() || body[j].kind != TokenKind::LBrace {
        return None;
    }
    let catch_body_start = j + 1;
    depth = 1;
    k = catch_body_start;
    while k < body.len() {
        match body[k].kind {
            TokenKind::LBrace => depth += 1,
            TokenKind::RBrace => {
                depth -= 1;
                if depth == 0 {
                    let catch_body = &body[catch_body_start..k];
                    if !is_simple_stmts_body(catch_body) {
                        return None;
                    }
                    return Some(k + 1);
                }
            }
            _ => {}
        }
        k += 1;
    }
    None
}

fn take_return_simple(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || body[0].kind != TokenKind::Return {
        return None;
    }
    // `return` <call-simple> [`;`] (incl. Type.method / recv.method — W900)
    if let Some(end) = take_call_simple(body, 1) {
        return if body.len() > end && body[end].kind == TokenKind::Semicolon {
            Some(end + 1)
        } else {
            Some(end)
        };
    }
    // W900: `return` <member-access> [`;`]
    if let Some(end) = take_member_access_simple(body, 1) {
        return if body.len() > end && body[end].kind == TokenKind::Semicolon {
            Some(end + 1)
        } else {
            Some(end)
        };
    }
    // W900: `return` <self.member> [`;`]
    if let Some(end) = take_self_member_access(body, 1) {
        return if body.len() > end && body[end].kind == TokenKind::Semicolon {
            Some(end + 1)
        } else {
            Some(end)
        };
    }
    // `return` <simple> [`;`]
    if body.len() >= 2 && is_simple_value(&body[1].kind) {
        if matches!(body[1].kind, TokenKind::Identifier(_))
            && body.len() >= 3
            && matches!(body[2].kind, TokenKind::LParen | TokenKind::Dot)
        {
            return None;
        }
        return if body.len() >= 3 && body[2].kind == TokenKind::Semicolon {
            Some(3)
        } else {
            Some(2)
        };
    }
    // Bare `return;` or bare `return`
    if body.len() >= 2 && body[1].kind == TokenKind::Semicolon {
        Some(2)
    } else {
        Some(1)
    }
}

/// W888: `self (. Ident)+ = <simple|call-simple> [`;`]?` — chained Dot OK; no `+=` / non-self.
fn take_self_field_assign(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || body[0].kind != TokenKind::SelfKw {
        return None;
    }
    let mut i = 1usize;
    let mut dots = 0usize;
    loop {
        if i >= body.len() || body[i].kind != TokenKind::Dot {
            break;
        }
        i += 1;
        if i >= body.len() || !matches!(body[i].kind, TokenKind::Identifier(_)) {
            return None;
        }
        i += 1;
        dots += 1;
    }
    if dots == 0 {
        return None;
    }
    // Reject compound `self.x +=` (PlusEqual etc.); only bare Assign.
    if i >= body.len() || body[i].kind != TokenKind::Assign {
        return None;
    }
    i += 1;
    take_member_assign_rhs(body, i)
}

/// W889: `Ident (. Ident)+ = <simple|call-simple> [`;`]?` — non-self member Assign only (compound → take_field_compound_assign).
fn take_member_field_assign(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || !matches!(body[0].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut i = 1usize;
    let mut dots = 0usize;
    loop {
        if i >= body.len() || body[i].kind != TokenKind::Dot {
            break;
        }
        i += 1;
        if i >= body.len() || !matches!(body[i].kind, TokenKind::Identifier(_)) {
            return None;
        }
        i += 1;
        dots += 1;
    }
    if dots == 0 {
        return None;
    }
    // Reject compound `box.x +=`; only bare Assign.
    if i >= body.len() || body[i].kind != TokenKind::Assign {
        return None;
    }
    i += 1;
    take_member_assign_rhs(body, i)
}

/// Shared RHS for self/member field assign: `<simple|call-simple|-int> [`;`]?`.
fn take_member_assign_rhs(body: &[&Token], i: usize) -> Option<usize> {
    let after_rhs = if let Some(end) = take_call_simple(body, i) {
        end
    } else if i < body.len()
        && body[i].kind == TokenKind::Minus
        && i + 1 < body.len()
        && matches!(body[i + 1].kind, TokenKind::Integer(_, _))
    {
        i + 2
    } else if i < body.len() && is_simple_value(&body[i].kind) {
        if matches!(body[i].kind, TokenKind::Identifier(_))
            && i + 1 < body.len()
            && matches!(body[i + 1].kind, TokenKind::LParen | TokenKind::Dot)
        {
            return None;
        }
        i + 1
    } else {
        return None;
    };
    if body.len() > after_rhs && body[after_rhs].kind == TokenKind::Semicolon {
        Some(after_rhs + 1)
    } else {
        Some(after_rhs)
    }
}

/// W890–W892: `(SelfKw | Ident) (. Ident)+ (+=|-=|*=|/=|%=) <simple|call-simple> [`;`]?`.
fn take_field_compound_assign(body: &[&Token]) -> Option<usize> {
    if body.is_empty() {
        return None;
    }
    let start_ok = body[0].kind == TokenKind::SelfKw
        || matches!(body[0].kind, TokenKind::Identifier(_));
    if !start_ok {
        return None;
    }
    let mut i = 1usize;
    let mut dots = 0usize;
    loop {
        if i >= body.len() || body[i].kind != TokenKind::Dot {
            break;
        }
        i += 1;
        if i >= body.len() || !matches!(body[i].kind, TokenKind::Identifier(_)) {
            return None;
        }
        i += 1;
        dots += 1;
    }
    if dots == 0 {
        return None;
    }
    // Five compound ops; index / nested / member-index → take_index_assign / take_member_index_assign.
    if i >= body.len()
        || !matches!(
            body[i].kind,
            TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::AmpEqual
                | TokenKind::PipeEqual
                | TokenKind::CaretEqual
                | TokenKind::LessLessEqual
                | TokenKind::GreaterGreaterEqual
        )
    {
        return None;
    }
    i += 1;
    take_member_assign_rhs(body, i)
}

/// W893: `Ident (+=|-=|*=|/=|%=) <simple|call-simple> [`;`]?` — bare ident compound; no Dot.
fn take_ident_compound_assign(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || !matches!(body[0].kind, TokenKind::Identifier(_)) {
        return None;
    }
    // Must not be member chain (Ident . …).
    if body.len() > 1 && body[1].kind == TokenKind::Dot {
        return None;
    }
    if body.len() < 2
        || !matches!(
            body[1].kind,
            TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::AmpEqual
                | TokenKind::PipeEqual
                | TokenKind::CaretEqual
                | TokenKind::LessLessEqual
                | TokenKind::GreaterGreaterEqual
        )
    {
        return None;
    }
    take_member_assign_rhs(body, 2)
}

/// W894–W898: `Ident [ <simple> ] ([ <simple> ])? (=|+=|-=|*=|/=|%=) <simple|call-simple> [`;`]?`
/// Nested second dim accepts Assign + five compounds (W898). Member-index → take_member_index_assign; depth≥3 out.
fn take_index_assign(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || !matches!(body[0].kind, TokenKind::Identifier(_)) {
        return None;
    }
    if body.len() < 5 || body[1].kind != TokenKind::LBracket {
        return None;
    }
    // First index must be bare simple (not Ident `(`).
    if !is_simple_value(&body[2].kind) {
        return None;
    }
    if matches!(body[2].kind, TokenKind::Identifier(_))
        && body.len() > 3
        && body[3].kind == TokenKind::LParen
    {
        return None;
    }
    if body[3].kind != TokenKind::RBracket {
        return None;
    }
    let mut i = 4;
    // Optional second dim: `[` simple `]` then Assign or five compounds (not depth≥3).
    if i < body.len() && body[i].kind == TokenKind::LBracket {
        if i + 2 >= body.len() {
            return None;
        }
        if !is_simple_value(&body[i + 1].kind) {
            return None;
        }
        if matches!(body[i + 1].kind, TokenKind::Identifier(_))
            && i + 2 < body.len()
            && body[i + 2].kind == TokenKind::LParen
        {
            return None;
        }
        if body[i + 2].kind != TokenKind::RBracket {
            return None;
        }
        i += 3;
        if i >= body.len()
            || !matches!(
                body[i].kind,
                TokenKind::Assign
                    | TokenKind::PlusEqual
                    | TokenKind::MinusEqual
                    | TokenKind::StarEqual
                    | TokenKind::SlashEqual
                    | TokenKind::PercentEqual
                    | TokenKind::AmpEqual
                    | TokenKind::PipeEqual
                    | TokenKind::CaretEqual
                    | TokenKind::LessLessEqual
                    | TokenKind::GreaterGreaterEqual
            )
        {
            return None;
        }
        i += 1;
        return take_member_assign_rhs(body, i);
    }
    // Single dim: Assign or five compound ops.
    if i >= body.len()
        || !matches!(
            body[i].kind,
            TokenKind::Assign
                | TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::AmpEqual
                | TokenKind::PipeEqual
                | TokenKind::CaretEqual
                | TokenKind::LessLessEqual
                | TokenKind::GreaterGreaterEqual
        )
    {
        return None;
    }
    i += 1;
    take_member_assign_rhs(body, i)
}

/// W897: `(SelfKw | Ident) (. Ident)+ [ <simple> ] = <simple|call-simple> [`;`]?`
/// Member-index Assign only (no compound / no nested second dim on member object).
fn take_member_index_assign(body: &[&Token]) -> Option<usize> {
    if body.is_empty()
        || !matches!(
            body[0].kind,
            TokenKind::SelfKw | TokenKind::Identifier(_)
        )
    {
        return None;
    }
    let mut i = 1usize;
    let mut dots = 0usize;
    loop {
        if i >= body.len() || body[i].kind != TokenKind::Dot {
            break;
        }
        i += 1;
        if i >= body.len() || !matches!(body[i].kind, TokenKind::Identifier(_)) {
            return None;
        }
        i += 1;
        dots += 1;
    }
    if dots == 0 {
        return None;
    }
    // `[` <simple> `]` then only Assign.
    if i + 2 >= body.len() || body[i].kind != TokenKind::LBracket {
        return None;
    }
    if !is_simple_value(&body[i + 1].kind) {
        return None;
    }
    if matches!(body[i + 1].kind, TokenKind::Identifier(_))
        && i + 2 < body.len()
        && body[i + 2].kind == TokenKind::LParen
    {
        return None;
    }
    if body[i + 2].kind != TokenKind::RBracket {
        return None;
    }
    i += 3;
    if i >= body.len() || body[i].kind != TokenKind::Assign {
        return None;
    }
    i += 1;
    take_member_assign_rhs(body, i)
}

/// Consume compare-simple at `tokens[i]`: `simple RelOp simple`
/// RelOp = `<`|`>`|`<=`|`>=` (W836). Rejects logic `&&`/`||`. Returns index after RHS, or `None`.
fn take_compare_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || !is_simple_value(&tokens[i].kind) {
        return None;
    }
    // Ident followed by `(` is call, not compare LHS.
    if matches!(tokens[i].kind, TokenKind::Identifier(_))
        && i + 1 < tokens.len()
        && tokens[i + 1].kind == TokenKind::LParen
    {
        return None;
    }
    let j = i + 1;
    if j >= tokens.len() {
        return None;
    }
    match tokens[j].kind {
        TokenKind::Less | TokenKind::Greater | TokenKind::LessEqual | TokenKind::GreaterEqual => {}
        _ => return None,
    }
    let k = j + 1;
    if k >= tokens.len() || !is_simple_value(&tokens[k].kind) {
        return None;
    }
    if matches!(tokens[k].kind, TokenKind::Identifier(_))
        && k + 1 < tokens.len()
        && tokens[k + 1].kind == TokenKind::LParen
    {
        return None;
    }
    Some(k + 1)
}

/// Bare simple operand (not Ident `(`). Returns index after operand, or `None`.
fn take_simple_operand(tokens: &[&Token], i: usize) -> Option<usize> {
    if i >= tokens.len() || !is_simple_value(&tokens[i].kind) {
        return None;
    }
    if matches!(tokens[i].kind, TokenKind::Identifier(_))
        && i + 1 < tokens.len()
        && tokens[i + 1].kind == TokenKind::LParen
    {
        return None;
    }
    Some(i + 1)
}

/// Max paren nesting in while-cond / logic operands (W871: `(((x)))` OK; `((((x))))` rejected; cap frozen).
const PAREN_NEST_MAX: usize = 3;

/// Unary-not with Bang stack depth bound (W870: depth≤4 → `!!!!x`; reject `!!!!!`).
/// `paren_budget` is remaining nest budget for paren operands (0 = no paren operand).
fn take_unary_not_at(
    tokens: &[&Token],
    i: usize,
    bang_depth: usize,
    paren_budget: usize,
) -> Option<usize> {
    if bang_depth > 4 || i >= tokens.len() || tokens[i].kind != TokenKind::Bang {
        return None;
    }
    let next = i + 1;
    if next < tokens.len() && tokens[next].kind == TokenKind::Bang {
        return take_unary_not_at(tokens, next, bang_depth + 1, paren_budget);
    }
    take_paren_group_at(tokens, next, paren_budget)
        .or_else(|| take_compare_simple(tokens, next))
        .or_else(|| take_call_simple(tokens, next))
        .or_else(|| take_simple_operand(tokens, next))
}

/// Unary-not: `!`/`!!`/`!!!`/`!!!!` + (paren-group | compare | call-simple | bare simple). Rejects `!!!!!` (W870).
fn take_unary_not_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    take_unary_not_at(tokens, i, 1, PAREN_NEST_MAX)
}

/// Unary-neg with Minus stack depth bound (W870: Neg cap frozen ≤18 → `------------------x`; reject `-------------------`; ladder closed).
/// `paren_budget` is remaining nest budget for paren operands (0 = no paren operand).
fn take_unary_neg_at(
    tokens: &[&Token],
    i: usize,
    neg_depth: usize,
    paren_budget: usize,
) -> Option<usize> {
    if neg_depth > 18 || i >= tokens.len() || tokens[i].kind != TokenKind::Minus {
        return None;
    }
    let next = i + 1;
    if next < tokens.len() && tokens[next].kind == TokenKind::Minus {
        return take_unary_neg_at(tokens, next, neg_depth + 1, paren_budget);
    }
    take_paren_group_at(tokens, next, paren_budget)
        .or_else(|| take_compare_simple(tokens, next))
        .or_else(|| take_call_simple(tokens, next))
        .or_else(|| take_simple_operand(tokens, next))
}

/// Unary-neg: `-`/`--`/`---`/`----`/`-----`/`------`/`-------`/`--------`/`---------`/`----------`/`-----------`/`------------`/`-------------`/`--------------`/`---------------`/`----------------`/`-----------------`/`------------------` + (paren-group | compare | call-simple | bare simple). Rejects `-------------------` (W870 Neg cap frozen).
fn take_unary_neg_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    take_unary_neg_at(tokens, i, 1, PAREN_NEST_MAX)
}

/// Paren group with nest budget (W871: entry budget=`PAREN_NEST_MAX` → `(((…)))` OK).
fn take_paren_group_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    take_paren_group_at(tokens, i, PAREN_NEST_MAX)
}

fn take_paren_group_at(tokens: &[&Token], i: usize, nest_budget: usize) -> Option<usize> {
    if nest_budget == 0 || i >= tokens.len() || tokens[i].kind != TokenKind::LParen {
        return None;
    }
    let inner_budget = nest_budget - 1;
    // Inner: logic / unary / nested paren / call / compare / simple.
    let after_inner = take_logic_simple_budget(tokens, i + 1, inner_budget)
        .or_else(|| take_unary_not_at(tokens, i + 1, 1, inner_budget))
        .or_else(|| take_unary_neg_at(tokens, i + 1, 1, inner_budget))
        .or_else(|| take_paren_group_at(tokens, i + 1, nest_budget - 1))
        .or_else(|| take_call_simple(tokens, i + 1))
        .or_else(|| take_compare_simple(tokens, i + 1))
        .or_else(|| take_simple_operand(tokens, i + 1))?;
    if after_inner >= tokens.len() || tokens[after_inner].kind != TokenKind::RParen {
        return None;
    }
    Some(after_inner + 1)
}

/// Logic operand with remaining paren nest budget.
fn take_logic_operand_budget(tokens: &[&Token], i: usize, paren_budget: usize) -> Option<usize> {
    take_paren_group_at(tokens, i, paren_budget)
        .or_else(|| take_unary_not_at(tokens, i, 1, paren_budget))
        .or_else(|| take_unary_neg_at(tokens, i, 1, paren_budget))
        .or_else(|| take_compare_simple(tokens, i))
        .or_else(|| take_call_simple(tokens, i))
        .or_else(|| take_simple_operand(tokens, i))
}

/// Logic operand: paren(nest≤3) | unary-not | unary-neg | compare | call-simple | bare simple (W871).
fn take_logic_operand(tokens: &[&Token], i: usize) -> Option<usize> {
    take_logic_operand_budget(tokens, i, PAREN_NEST_MAX)
}

fn take_logic_chain_after(
    tokens: &[&Token],
    mut j: usize,
    operand: fn(&[&Token], usize) -> Option<usize>,
) -> Option<usize> {
    if j >= tokens.len() {
        return None;
    }
    match tokens[j].kind {
        TokenKind::AndAnd | TokenKind::OrOr => {}
        _ => return None,
    }
    j = operand(tokens, j + 1)?;
    loop {
        if j >= tokens.len() {
            return Some(j);
        }
        match tokens[j].kind {
            TokenKind::AndAnd | TokenKind::OrOr => {
                j = operand(tokens, j + 1)?;
            }
            _ => return Some(j),
        }
    }
}

/// Left-assoc `&&`/`||` chain with full operands (incl. paren nest≤3) (W871).
fn take_logic_simple(tokens: &[&Token], i: usize) -> Option<usize> {
    let j = take_logic_operand(tokens, i)?;
    take_logic_chain_after(tokens, j, take_logic_operand)
}

fn take_logic_simple_budget(tokens: &[&Token], i: usize, paren_budget: usize) -> Option<usize> {
    let j = take_logic_operand_budget(tokens, i, paren_budget)?;
    // Chain operands keep the same paren budget.
    if j >= tokens.len() {
        return None;
    }
    match tokens[j].kind {
        TokenKind::AndAnd | TokenKind::OrOr => {}
        _ => return None,
    }
    let mut k = take_logic_operand_budget(tokens, j + 1, paren_budget)?;
    loop {
        if k >= tokens.len() {
            return Some(k);
        }
        match tokens[k].kind {
            TokenKind::AndAnd | TokenKind::OrOr => {
                k = take_logic_operand_budget(tokens, k + 1, paren_budget)?;
            }
            _ => return Some(k),
        }
    }
}

/// Inner while/if body stmt: print / let / var / return / while-simple / if-simple (W836 + W877).
fn take_while_body_stmt(sig: &[&Token], i: usize) -> Option<usize> {
    // W901: while/if braced bodies share the full simple-stmt surface (try/catch,
    // bare call, compound assign, …) so Production Scope examples stay CST-green
    // with comment trivia — previously only print/let/var/return/while/if.
    take_simple_stmt(sig, i)
}

/// Brace-less while/if-then body: exactly one print / let / var / return (no nested while/if).
fn take_braceless_while_body(body: &[&Token], j: usize) -> Option<usize> {
    if j >= body.len() {
        return None;
    }
    let rest = &body[j..];
    if let Some(n) = take_print_simple_call(rest) {
        return Some(j + n);
    }
    if let Some(n) = take_let_simple_bind(rest) {
        return Some(j + n);
    }
    if let Some(n) = take_var_simple_bind(rest) {
        return Some(j + n);
    }
    if let Some(n) = take_return_simple(rest) {
        return Some(j + n);
    }
    None
}

/// Shared cond for while-simple / if-simple (W877).
fn take_while_or_if_cond(body: &[&Token], j: usize) -> Option<usize> {
    if let Some(end) = take_logic_simple(body, j) {
        Some(end)
    } else if let Some(end) = take_unary_not_simple(body, j) {
        Some(end)
    } else if let Some(end) = take_unary_neg_simple(body, j) {
        Some(end)
    } else if let Some(end) = take_paren_group_simple(body, j) {
        Some(end)
    } else if let Some(end) = take_call_simple(body, j) {
        Some(end)
    } else if let Some(end) = take_compare_simple(body, j) {
        Some(end)
    } else if j < body.len() && is_simple_value(&body[j].kind) {
        if matches!(body[j].kind, TokenKind::Identifier(_))
            && j + 1 < body.len()
            && body[j + 1].kind == TokenKind::LParen
        {
            return None;
        }
        Some(j + 1)
    } else {
        None
    }
}

/// Consume while-simple at `body[0..]`:
/// `while` (logic-simple|unary-not|unary-neg|paren-group|call-simple|compare-simple|simple) (`{` stmts `}` | print|let|var|return).
/// Logic cond = left-assoc `&&`/`||` of paren(nest≤3)|!|!!|!!!|!!!!|-|--|compare|call|simple (W871; no `((((…))))` / `!!!!!` / Neg>`------------------`).
/// Brace-less body = single non-while stmt. Returns tokens consumed, or `None`.
fn take_while_simple(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || body[0].kind != TokenKind::While {
        return None;
    }
    let mut j = 1usize;
    j = take_while_or_if_cond(body, j)?;
    // Braced body.
    if j < body.len() && body[j].kind == TokenKind::LBrace {
        j += 1;
        while j < body.len() && body[j].kind != TokenKind::RBrace {
            let Some(next) = take_while_body_stmt(body, j) else {
                return None;
            };
            j = next;
        }
        if j >= body.len() || body[j].kind != TokenKind::RBrace {
            return None;
        }
        return Some(j + 1);
    }
    // W843: brace-less single stmt body.
    take_braceless_while_body(body, j)
}

/// Consume if-simple at `body[0..]`:
/// `if` cond (`{` stmts `}` | print|let|var|return) [`else` (`{` stmts `}` | print|let|var|return|if-simple)].
/// Cond matches while-simple. Rejects `if case` / `if let` (W877; M0 still AST F001).
/// Brace-less `else` may be another if-simple (`else if`). Returns tokens consumed, or `None`.
fn take_if_simple(body: &[&Token]) -> Option<usize> {
    if body.is_empty() || body[0].kind != TokenKind::If {
        return None;
    }
    // Reject if case / if let (do not steal M0 / if-let into CST).
    if body.len() > 1 && matches!(body[1].kind, TokenKind::Case | TokenKind::Let) {
        return None;
    }
    let mut j = 1usize;
    j = take_while_or_if_cond(body, j)?;
    // Then body.
    if j < body.len() && body[j].kind == TokenKind::LBrace {
        j += 1;
        while j < body.len() && body[j].kind != TokenKind::RBrace {
            let Some(next) = take_while_body_stmt(body, j) else {
                return None;
            };
            j = next;
        }
        if j >= body.len() || body[j].kind != TokenKind::RBrace {
            return None;
        }
        j += 1;
    } else {
        j = take_braceless_while_body(body, j)?;
    }
    // Optional else.
    if j < body.len() && body[j].kind == TokenKind::Else {
        j += 1;
        if j < body.len() && body[j].kind == TokenKind::LBrace {
            j += 1;
            while j < body.len() && body[j].kind != TokenKind::RBrace {
                let Some(next) = take_while_body_stmt(body, j) else {
                    return None;
                };
                j = next;
            }
            if j >= body.len() || body[j].kind != TokenKind::RBrace {
                return None;
            }
            j += 1;
        } else if let Some(end) = take_braceless_while_body(body, j) {
            j = end;
        } else if let Some(n) = take_if_simple(&body[j..]) {
            // else if …
            j += n;
        } else {
            return None;
        }
    }
    Some(j)
}

/// One simple stmt at `sig[i..]`: print / let / var / return[-simple] / self|member.field[=|+=] / while-simple / if-simple (W835–W890).
/// Returns index after the stmt, or `None`.
fn take_simple_stmt(sig: &[&Token], i: usize) -> Option<usize> {
    if i >= sig.len() {
        return None;
    }
    let rest = &sig[i..];
    if let Some(n) = take_print_simple_call(rest) {
        return Some(i + n);
    }
    if let Some(n) = take_let_simple_bind(rest) {
        return Some(i + n);
    }
    if let Some(n) = take_var_simple_bind(rest) {
        return Some(i + n);
    }
    if let Some(n) = take_return_simple(rest) {
        return Some(i + n);
    }
    // W900: throw / try-catch (file_handle / IO facades).
    if let Some(n) = take_throw_simple(rest) {
        return Some(i + n);
    }
    if let Some(n) = take_try_catch_simple(rest) {
        return Some(i + n);
    }
    // W887–W888: self(.Ident)+ = <simple|call-simple>.
    if let Some(n) = take_self_field_assign(rest) {
        return Some(i + n);
    }
    // W889: Ident(.Ident)+ = <simple|call-simple> (non-self member assign).
    if let Some(n) = take_member_field_assign(rest) {
        return Some(i + n);
    }
    // W890–W892: (SelfKw|Ident)(.Ident)+ (+=|-=|*=|/=|%=) <simple|call-simple>.
    if let Some(n) = take_field_compound_assign(rest) {
        return Some(i + n);
    }
    // W893: Ident (+=|-=|*=|/=|%=) <simple|call-simple> (no Dot).
    if let Some(n) = take_ident_compound_assign(rest) {
        return Some(i + n);
    }
    // W897: (SelfKw|Ident)(.Ident)+ [ <simple> ] = <simple|call-simple>.
    if let Some(n) = take_member_index_assign(rest) {
        return Some(i + n);
    }
    // W894–W896: Ident [ <simple> ] ([ <simple> ])? (=|compound) <simple|call-simple>.
    if let Some(n) = take_index_assign(rest) {
        return Some(i + n);
    }
    // W835–W836: while-simple (nested + compare cond OK).
    if let Some(n) = take_while_simple(rest) {
        return Some(i + n);
    }
    // W877: if-simple (optional else / else-if).
    if let Some(n) = take_if_simple(rest) {
        return Some(i + n);
    }
    // W900: bare call-simple stmt (`__syx_hasher_combine(hasher, self)` / `Hasher.combine(…)`).
    if let Some(end) = take_call_simple(rest, 0) {
        let end = if end < rest.len() && rest[end].kind == TokenKind::Semicolon {
            end + 1
        } else {
            end
        };
        return Some(i + end);
    }
    None
}

/// Body: empty, or 1..N consecutive print / let / var / return / self|member.field= / while-simple / if-simple stmts.
fn is_simple_stmts_body(body: &[&Token]) -> bool {
    if body.is_empty() {
        return true;
    }
    let mut i = 0usize;
    while i < body.len() {
        let Some(next) = take_simple_stmt(body, i) else {
            return false;
        };
        i = next;
    }
    true
}

/// Consume one Named-generic type arg: NestedNamed | Amp family | Function TypeExpr (W833–W834).
fn take_param_generic_arg(sig: &[&Token], i: usize) -> Option<usize> {
    if i >= sig.len() {
        return None;
    }
    if sig[i].kind == TokenKind::Amp {
        return take_amp_reference_named(sig, i);
    }
    if sig[i].kind == TokenKind::LParen {
        // W834: Function TypeExpr as generic arg (`Foo<(Int)->Bool>` / `Bar<()>`).
        return take_function_type_expr(sig, i);
    }
    take_param_nested_named(sig, i)
}

/// Consume NestedNamed for func param/return generic: `Ident` [`Less` Arg (`,` Arg)* `Greater`].
/// W824: nest; W825: multi-arg; W833: Amp arg; W834: Function arg; no trailing `?` (outer only).
fn take_param_nested_named(sig: &[&Token], i: usize) -> Option<usize> {
    if i >= sig.len() || !matches!(sig[i].kind, TokenKind::Identifier(_)) {
        return None;
    }
    let mut j = i + 1;
    if j < sig.len() && sig[j].kind == TokenKind::Less {
        j += 1;
        j = take_param_generic_arg(sig, j)?;
        while j < sig.len() && sig[j].kind == TokenKind::Comma {
            j += 1;
            j = take_param_generic_arg(sig, j)?;
        }
        if j >= sig.len() || sig[j].kind != TokenKind::Greater {
            return None;
        }
        j += 1;
    }
    Some(j)
}

/// Consume one param type after `Colon` at `sig[j]`: `Ident` [`<` Arg (`,` Arg)* `>`] [`?`] | `&` [`Lifetime`] [`mut`] `Ident` [`?`].
/// W817–W825 / W833. Nested/multi-arg Named generic OK; Arg may be Amp; does not swallow Function as generic arg.
/// Returns index after the type, or `None`.
fn take_func_param_type(sig: &[&Token], j: usize) -> Option<usize> {
    if j >= sig.len() || sig[j].kind != TokenKind::Colon {
        return None;
    }
    let mut k = j + 1;
    if k >= sig.len() {
        return None;
    }
    // W819–W822: `&` [`Lifetime`] [`mut`] `Ident` [`?`].
    if sig[k].kind == TokenKind::Amp {
        k += 1;
        if k >= sig.len() {
            return None;
        }
        // W821: optional lifetime (`&'a Ident` / `&'a mut Ident`).
        if matches!(sig[k].kind, TokenKind::Lifetime(_)) {
            k += 1;
            if k >= sig.len() {
                return None;
            }
        }
        // Optional `mut` keyword (Identifier "mut").
        if matches!(&sig[k].kind, TokenKind::Identifier(name) if name.as_str() == "mut") {
            k += 1;
            if k >= sig.len() {
                return None;
            }
        }
        if !matches!(sig[k].kind, TokenKind::Identifier(_)) {
            return None;
        }
        k += 1;
        // W822: optional trailing `?` after Amp family (`&Int?` / `&mut Int?` / `&'a Int?`).
        if k < sig.len() && sig[k].kind == TokenKind::Question {
            k += 1;
        }
        return Some(k);
    }
    // W900: dictionary type `[ Ident : Ident ]` (e.g. `[String: Int]` in builtins).
    if sig[k].kind == TokenKind::LBracket {
        k += 1;
        if k >= sig.len() || !matches!(sig[k].kind, TokenKind::Identifier(_)) {
            return None;
        }
        k += 1;
        if k >= sig.len() || sig[k].kind != TokenKind::Colon {
            return None;
        }
        k += 1;
        if k >= sig.len() || !matches!(sig[k].kind, TokenKind::Identifier(_)) {
            return None;
        }
        k += 1;
        if k >= sig.len() || sig[k].kind != TokenKind::RBracket {
            return None;
        }
        k += 1;
        if k < sig.len() && sig[k].kind == TokenKind::Question {
            k += 1;
        }
        return Some(k);
    }
    // W817–W825 / W833: Named Ident [`< Arg (, Arg)* >`] [`?`] (Arg = NestedNamed | Amp).
    if !matches!(sig[k].kind, TokenKind::Identifier(_)) {
        return None;
    }
    k += 1;
    if k < sig.len() && sig[k].kind == TokenKind::Less {
        k += 1;
        k = take_param_generic_arg(sig, k)?;
        while k < sig.len() && sig[k].kind == TokenKind::Comma {
            k += 1;
            k = take_param_generic_arg(sig, k)?;
        }
        if k >= sig.len() || sig[k].kind != TokenKind::Greater {
            return None;
        }
        k += 1;
    }
    if k < sig.len() && sig[k].kind == TokenKind::Question {
        k += 1;
    }
    Some(k)
}

/// Consume one `[export] [access] [static|async] func Ident [<Ident(, Ident)*>] ( … ) [-> …] { … }` from `sig[i..]`.
/// W816–W833: bare/typed params + optional return (Named multi-arg/nested generic, Amp 族, or Function TypeExpr); empty `()` still OK.
/// W875: optional access before `func` (Public|Private|Internal|FilePrivate).
/// W876: optional Static|Async after access (e.g. `public static func`); still reject static+async stack.
/// W878: optional Export before access (e.g. `export func` / `export static func`).
/// W883: `export {…}` list handled by `take_export_list_shell` (try before this).
/// Rejects depth>2 nested-call / `!!!!!` / Neg>`------------------` while-cond / bare Amp-generic return (`&Ident<…>`).
/// Returns `(body_start, body_end_exclusive, next_i)`, or `None`.
fn take_func_shell(sig: &[&Token], i: usize) -> Option<(usize, usize, usize)> {
    let mut j = i;
    // W878: optional export before access/func.
    if j < sig.len() && sig[j].kind == TokenKind::Export {
        j += 1;
    }
    // W875: optional access before `func`.
    if j < sig.len()
        && matches!(
            sig[j].kind,
            TokenKind::Public
                | TokenKind::Private
                | TokenKind::Internal
                | TokenKind::FilePrivate
        )
    {
        j += 1;
    }
    // W876: optional static|async after access (0..1; no stack).
    if j < sig.len() && matches!(sig[j].kind, TokenKind::Static | TokenKind::Async) {
        j += 1;
    }
    if j + 4 > sig.len() {
        return None;
    }
    if sig[j].kind != TokenKind::Func {
        return None;
    }
    if !matches!(sig[j + 1].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 2;
    // W840: optional `< Ident (, Ident)* >` before `(`.
    if j < sig.len() && sig[j].kind == TokenKind::Less {
        j += 1;
        if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
            return None;
        }
        j += 1;
        while j < sig.len() && sig[j].kind == TokenKind::Comma {
            j += 1;
            if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
                return None;
            }
            j += 1;
        }
        if j >= sig.len() || sig[j].kind != TokenKind::Greater {
            return None;
        }
        j += 1;
    }
    if j >= sig.len() || sig[j].kind != TokenKind::LParen {
        return None;
    }
    j += 1;
    // W816–W825 / W833 / W900: optional params —
    // `Ident [: type]` or external-label `Ident Ident : type` (, …)*
    if j < sig.len() && matches!(sig[j].kind, TokenKind::Identifier(_)) {
        j += 1;
        // W900: optional internal name after external label (`into hasher: Hasher`).
        if j < sig.len()
            && matches!(sig[j].kind, TokenKind::Identifier(_))
            && j + 1 < sig.len()
            && sig[j + 1].kind == TokenKind::Colon
        {
            j += 1;
        }
        if j < sig.len() && sig[j].kind == TokenKind::Colon {
            j = take_func_param_type(sig, j)?;
        }
        while j + 1 < sig.len()
            && sig[j].kind == TokenKind::Comma
            && matches!(sig[j + 1].kind, TokenKind::Identifier(_))
        {
            j += 2;
            if j < sig.len()
                && matches!(sig[j].kind, TokenKind::Identifier(_))
                && j + 1 < sig.len()
                && sig[j + 1].kind == TokenKind::Colon
            {
                j += 1;
            }
            if j < sig.len() && sig[j].kind == TokenKind::Colon {
                j = take_func_param_type(sig, j)?;
            }
        }
    }
    if j >= sig.len() || sig[j].kind != TokenKind::RParen {
        return None;
    }
    j += 1;
    // W826–W833: optional `-> Ident [< Arg (, Arg)* >] [?]` | Amp 族[?] | Function TypeExpr before `{`.
    if j < sig.len() && sig[j].kind == TokenKind::Arrow {
        j += 1;
        if j >= sig.len() {
            return None;
        }
        if sig[j].kind == TokenKind::Amp {
            // W827–W830: `&Ident[?]` | `&mut Ident[?]` | `&'a Ident[?]` | `&'a mut Ident[?]`; no generic.
            j += 1;
            if j >= sig.len() {
                return None;
            }
            // W829: optional lifetime (`&'a Ident` / `&'a mut Ident`).
            if matches!(sig[j].kind, TokenKind::Lifetime(_)) {
                j += 1;
                if j >= sig.len() {
                    return None;
                }
            }
            // W828: optional `mut` keyword before type Ident.
            if matches!(&sig[j].kind, TokenKind::Identifier(name) if name.as_str() == "mut") {
                j += 1;
                if j >= sig.len() {
                    return None;
                }
            }
            if !matches!(sig[j].kind, TokenKind::Identifier(_)) {
                return None;
            }
            j += 1;
            // W830: optional trailing `?` after Amp family.
            if j < sig.len() && sig[j].kind == TokenKind::Question {
                j += 1;
            }
            // Do not swallow trailing `<` (bare Amp-generic return remains CST001).
            if j < sig.len() && sig[j].kind == TokenKind::Less {
                return None;
            }
        } else if sig[j].kind == TokenKind::LParen {
            // W833: Function TypeExpr return (`-> (Int) -> Bool` / `-> ()`).
            j = take_function_type_expr(sig, j)?;
        } else if matches!(sig[j].kind, TokenKind::Identifier(_)) {
            j += 1;
            // W831–W833: optional Named generic `< Arg (, Arg)* >` (nested/multi-arg/Amp-arg OK).
            if j < sig.len() && sig[j].kind == TokenKind::Less {
                j += 1;
                j = take_param_generic_arg(sig, j)?;
                while j < sig.len() && sig[j].kind == TokenKind::Comma {
                    j += 1;
                    j = take_param_generic_arg(sig, j)?;
                }
                if j >= sig.len() || sig[j].kind != TokenKind::Greater {
                    return None;
                }
                j += 1;
            }
            if j < sig.len() && sig[j].kind == TokenKind::Question {
                j += 1;
            }
        } else {
            return None;
        }
    }
    // W839: optional bare `throws` before optional `requires` / `{`.
    if j < sig.len() && sig[j].kind == TokenKind::Throws {
        j += 1;
    }
    // W837: optional `requires Pure|IO` before optional where / `{`.
    if j < sig.len() && sig[j].kind == TokenKind::Requires {
        j += 1;
        if j >= sig.len() {
            return None;
        }
        match sig[j].kind {
            TokenKind::Pure | TokenKind::Io => {
                j += 1;
            }
            _ => return None,
        }
    }
    // W841: optional `where Ident : Ident (, Ident : Ident)*` before `{`.
    if j < sig.len() && sig[j].kind == TokenKind::Where {
        j += 1;
        // First pred: Ident : Ident
        if j + 2 >= sig.len()
            || !matches!(sig[j].kind, TokenKind::Identifier(_))
            || sig[j + 1].kind != TokenKind::Colon
            || !matches!(sig[j + 2].kind, TokenKind::Identifier(_))
        {
            return None;
        }
        j += 3;
        while j < sig.len() && sig[j].kind == TokenKind::Comma {
            j += 1;
            if j + 2 >= sig.len()
                || !matches!(sig[j].kind, TokenKind::Identifier(_))
                || sig[j + 1].kind != TokenKind::Colon
                || !matches!(sig[j + 2].kind, TokenKind::Identifier(_))
            {
                return None;
            }
            j += 3;
        }
    }
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    let body_start = j + 1;
    let mut depth = 1usize;
    let mut k = body_start;
    while k < sig.len() {
        match sig[k].kind {
            TokenKind::LBrace => depth += 1,
            TokenKind::RBrace => {
                depth -= 1;
                if depth == 0 {
                    return Some((body_start, k, k + 1));
                }
            }
            _ => {}
        }
        k += 1;
    }
    None
}

/// W879/W900: class stored field `[access?] var Ident : Ident[?] [= <simple>]?`.
fn take_class_stored_var(sig: &[&Token], i: usize) -> Option<usize> {
    let mut j = i;
    if j < sig.len()
        && matches!(
            sig[j].kind,
            TokenKind::Public
                | TokenKind::Private
                | TokenKind::Internal
                | TokenKind::FilePrivate
        )
    {
        j += 1;
    }
    if j + 3 >= sig.len() {
        return None;
    }
    if sig[j].kind != TokenKind::Var {
        return None;
    }
    if !matches!(sig[j + 1].kind, TokenKind::Identifier(_)) {
        return None;
    }
    if sig[j + 2].kind != TokenKind::Colon {
        return None;
    }
    if !matches!(sig[j + 3].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 4;
    if j < sig.len() && sig[j].kind == TokenKind::Question {
        j += 1;
    }
    // W900: optional `= <simple>` | `= - <int>` initializer.
    if j < sig.len() && sig[j].kind == TokenKind::Assign {
        j += 1;
        if j < sig.len()
            && sig[j].kind == TokenKind::Minus
            && j + 1 < sig.len()
            && matches!(sig[j + 1].kind, TokenKind::Integer(_, _))
        {
            j += 2;
        } else if j < sig.len() && is_simple_value(&sig[j].kind) {
            if matches!(sig[j].kind, TokenKind::Identifier(_))
                && j + 1 < sig.len()
                && matches!(sig[j + 1].kind, TokenKind::LParen | TokenKind::Dot)
            {
                return None;
            }
            j += 1;
        } else {
            return None;
        }
    }
    if j < sig.len() && sig[j].kind == TokenKind::Semicolon {
        j += 1;
    }
    Some(j)
}

/// W880: `init([Ident[: type](, …)*]) { simple-stmts }` — body same as func subset; W887–W888 allows `self.field=` / nested.
/// Returns `(body_start, body_end_exclusive, next_i)`, or `None`.
fn take_class_init(sig: &[&Token], i: usize) -> Option<(usize, usize, usize)> {
    let mut j = i;
    if j >= sig.len() || sig[j].kind != TokenKind::Init {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::LParen {
        return None;
    }
    j += 1;
    // Optional Ident [: type] (, Ident [: type])* — same narrow as take_func_shell params.
    if j < sig.len() && matches!(sig[j].kind, TokenKind::Identifier(_)) {
        j += 1;
        if j < sig.len() && sig[j].kind == TokenKind::Colon {
            j = take_func_param_type(sig, j)?;
        }
        while j + 1 < sig.len()
            && sig[j].kind == TokenKind::Comma
            && matches!(sig[j + 1].kind, TokenKind::Identifier(_))
        {
            j += 2;
            if j < sig.len() && sig[j].kind == TokenKind::Colon {
                j = take_func_param_type(sig, j)?;
            }
        }
    }
    if j >= sig.len() || sig[j].kind != TokenKind::RParen {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    let body_start = j + 1;
    let mut depth = 1usize;
    let mut k = body_start;
    while k < sig.len() {
        match sig[k].kind {
            TokenKind::LBrace => depth += 1,
            TokenKind::RBrace => {
                depth -= 1;
                if depth == 0 {
                    return Some((body_start, k, k + 1));
                }
            }
            _ => {}
        }
        k += 1;
    }
    None
}

/// W886: `deinit { simple-stmts }` — body same as func subset; W887–W888 allows `self.field=` / nested.
/// Returns `(body_start, body_end_exclusive, next_i)`, or `None`.
fn take_class_deinit(sig: &[&Token], i: usize) -> Option<(usize, usize, usize)> {
    let mut j = i;
    if j >= sig.len() || sig[j].kind != TokenKind::Deinit {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    let body_start = j + 1;
    let mut depth = 1usize;
    let mut k = body_start;
    while k < sig.len() {
        match sig[k].kind {
            TokenKind::LBrace => depth += 1,
            TokenKind::RBrace => {
                depth -= 1;
                if depth == 0 {
                    return Some((body_start, k, k + 1));
                }
            }
            _ => {}
        }
        k += 1;
    }
    None
}

/// Shared: `(` [Ident `:` type (, Ident `:` type)*]? `)` starting at `sig[j]` (`LParen`).
/// Field types reuse `take_func_param_type`. Empty `()` OK.
fn take_paren_typed_fields(sig: &[&Token], j: usize) -> Option<usize> {
    if j >= sig.len() || sig[j].kind != TokenKind::LParen {
        return None;
    }
    let mut k = j + 1;
    if k < sig.len() && matches!(sig[k].kind, TokenKind::Identifier(_)) {
        k += 1;
        if k >= sig.len() || sig[k].kind != TokenKind::Colon {
            return None;
        }
        k = take_func_param_type(sig, k)?;
        while k + 1 < sig.len()
            && sig[k].kind == TokenKind::Comma
            && matches!(sig[k + 1].kind, TokenKind::Identifier(_))
        {
            k += 2;
            if k >= sig.len() || sig[k].kind != TokenKind::Colon {
                return None;
            }
            k = take_func_param_type(sig, k)?;
        }
    }
    if k >= sig.len() || sig[k].kind != TokenKind::RParen {
        return None;
    }
    Some(k + 1)
}

/// Shared: optional `< Ident (, Ident)* >` starting at `sig[j]` if `Less`, else leave `j` unchanged.
/// Same narrow as `take_func_shell` type params (W840).
fn take_optional_type_params(sig: &[&Token], j: usize) -> Option<usize> {
    if j >= sig.len() || sig[j].kind != TokenKind::Less {
        return Some(j);
    }
    let mut k = j + 1;
    if k >= sig.len() || !matches!(sig[k].kind, TokenKind::Identifier(_)) {
        return None;
    }
    k += 1;
    while k < sig.len() && sig[k].kind == TokenKind::Comma {
        k += 1;
        if k >= sig.len() || !matches!(sig[k].kind, TokenKind::Identifier(_)) {
            return None;
        }
        k += 1;
    }
    if k >= sig.len() || sig[k].kind != TokenKind::Greater {
        return None;
    }
    Some(k + 1)
}

/// Shared: optional `where Ident : Ident (, Ident : Ident)*` starting at `sig[j]` if `Where`.
/// Same narrow as `take_func_shell` where (W841). Returns index after preds, or `None` on malformed.
fn take_optional_where_preds(sig: &[&Token], j: usize) -> Option<usize> {
    if j >= sig.len() || sig[j].kind != TokenKind::Where {
        return Some(j);
    }
    let mut k = j + 1;
    if k + 2 >= sig.len()
        || !matches!(sig[k].kind, TokenKind::Identifier(_))
        || sig[k + 1].kind != TokenKind::Colon
        || !matches!(sig[k + 2].kind, TokenKind::Identifier(_))
    {
        return None;
    }
    k += 3;
    while k < sig.len() && sig[k].kind == TokenKind::Comma {
        k += 1;
        if k + 2 >= sig.len()
            || !matches!(sig[k].kind, TokenKind::Identifier(_))
            || sig[k + 1].kind != TokenKind::Colon
            || !matches!(sig[k + 2].kind, TokenKind::Identifier(_))
        {
            return None;
        }
        k += 3;
    }
    Some(k)
}

/// W881/W882: `[access?] data class Ident [<T[,…]>] ( typed-fields? ) [where …]?`.
/// W882: optional type params + optional where. Returns index after clause, or `None`.
fn take_data_class_shell(sig: &[&Token], i: usize) -> Option<usize> {
    let mut j = i;
    if j < sig.len()
        && matches!(
            sig[j].kind,
            TokenKind::Public
                | TokenKind::Private
                | TokenKind::Internal
                | TokenKind::FilePrivate
        )
    {
        j += 1;
    }
    if j >= sig.len() || sig[j].kind != TokenKind::Data {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::Class {
        return None;
    }
    j += 1;
    if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    // W882: optional `< Ident (, Ident)* >`.
    j = take_optional_type_params(sig, j)?;
    j = take_paren_typed_fields(sig, j)?;
    // W882: optional `where Ident : Ident (, …)*`.
    j = take_optional_where_preds(sig, j)?;
    Some(j)
}

/// W881–W884: `[access?] final class Ident [<T[,…]>] [where …]? { variants }`.
/// W882: optional type params. W884: optional where (same narrow as data). Returns index after `}`, or `None`.
fn take_final_class_shell(sig: &[&Token], i: usize) -> Option<usize> {
    let mut j = i;
    if j < sig.len()
        && matches!(
            sig[j].kind,
            TokenKind::Public
                | TokenKind::Private
                | TokenKind::Internal
                | TokenKind::FilePrivate
        )
    {
        j += 1;
    }
    if j >= sig.len() || sig[j].kind != TokenKind::Final {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::Class {
        return None;
    }
    j += 1;
    if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    // W882: optional `< Ident (, Ident)* >`.
    j = take_optional_type_params(sig, j)?;
    // W884: optional `where Ident : Ident (, …)*`.
    j = take_optional_where_preds(sig, j)?;
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    j += 1;
    while j < sig.len() {
        if sig[j].kind == TokenKind::RBrace {
            return Some(j + 1);
        }
        if !matches!(sig[j].kind, TokenKind::Identifier(_)) {
            return None;
        }
        j += 1;
        if j < sig.len() && sig[j].kind == TokenKind::LParen {
            j = take_paren_typed_fields(sig, j)?;
        }
    }
    None
}

/// W879/W880/W886: `[access?] class Ident [: Ident (, Ident)*]? { members }` —
/// members = stored var / func / init / deinit; empty OK.
/// W886: multi-conformance `, Ident*` OK; `deinit { simple-stmts }` OK.
/// Still rejects compound `+=`. Returns index after `}`, or `None`.
fn take_class_shell(sig: &[&Token], i: usize) -> Option<usize> {
    let mut j = i;
    if j < sig.len()
        && matches!(
            sig[j].kind,
            TokenKind::Public
                | TokenKind::Private
                | TokenKind::Internal
                | TokenKind::FilePrivate
        )
    {
        j += 1;
    }
    if j >= sig.len() || sig[j].kind != TokenKind::Class {
        return None;
    }
    j += 1;
    if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    // W880/W886: optional `: Ident (, Ident)*` (single super or multi-conformance).
    if j < sig.len() && sig[j].kind == TokenKind::Colon {
        j += 1;
        if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
            return None;
        }
        j += 1;
        while j < sig.len() && sig[j].kind == TokenKind::Comma {
            j += 1;
            if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
                return None;
            }
            j += 1;
        }
    }
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    j += 1;
    while j < sig.len() {
        if sig[j].kind == TokenKind::RBrace {
            return Some(j + 1);
        }
        if let Some((body_start, body_end, next)) = take_func_shell(sig, j) {
            let body = &sig[body_start..body_end];
            if !is_simple_stmts_body(body) {
                return None;
            }
            j = next;
            continue;
        }
        if let Some((body_start, body_end, next)) = take_class_init(sig, j) {
            let body = &sig[body_start..body_end];
            if !is_simple_stmts_body(body) {
                return None;
            }
            j = next;
            continue;
        }
        if let Some((body_start, body_end, next)) = take_class_deinit(sig, j) {
            let body = &sig[body_start..body_end];
            if !is_simple_stmts_body(body) {
                return None;
            }
            j = next;
            continue;
        }
        if let Some(next) = take_class_stored_var(sig, j) {
            j = next;
            continue;
        }
        return None;
    }
    None
}

/// W900: `extension Ident { [func…]* }` — method bodies must be simple-stmts.
/// Returns index after `}`, or `None`.
fn take_extension_shell(sig: &[&Token], i: usize) -> Option<usize> {
    let mut j = i;
    if j >= sig.len() || sig[j].kind != TokenKind::Extension {
        return None;
    }
    j += 1;
    if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    j += 1;
    while j < sig.len() {
        if sig[j].kind == TokenKind::RBrace {
            return Some(j + 1);
        }
        if let Some((body_start, body_end, next)) = take_func_shell(sig, j) {
            let body = &sig[body_start..body_end];
            if !is_simple_stmts_body(body) {
                return None;
            }
            j = next;
            continue;
        }
        return None;
    }
    None
}

/// W883/W885: `export { [Ident [as Ident] (, Ident [as Ident])* [,]?] }` —
/// empty list OK; trailing comma OK; optional `as` rename.
/// Returns index after `}`, or `None`.
fn take_export_list_shell(sig: &[&Token], i: usize) -> Option<usize> {
    let mut j = i;
    if j >= sig.len() || sig[j].kind != TokenKind::Export {
        return None;
    }
    j += 1;
    if j >= sig.len() || sig[j].kind != TokenKind::LBrace {
        return None;
    }
    j += 1;
    // W885: empty `export { }` OK.
    if j < sig.len() && sig[j].kind == TokenKind::RBrace {
        return Some(j + 1);
    }
    // First Ident required when non-empty.
    if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
        return None;
    }
    j += 1;
    // W885: optional `as Ident` rename.
    if j < sig.len() && sig[j].kind == TokenKind::As {
        j += 1;
        if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
            return None;
        }
        j += 1;
    }
    while j < sig.len() && sig[j].kind == TokenKind::Comma {
        j += 1;
        // Trailing comma before `}`.
        if j < sig.len() && sig[j].kind == TokenKind::RBrace {
            return Some(j + 1);
        }
        if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
            return None;
        }
        j += 1;
        if j < sig.len() && sig[j].kind == TokenKind::As {
            j += 1;
            if j >= sig.len() || !matches!(sig[j].kind, TokenKind::Identifier(_)) {
                return None;
            }
            j += 1;
        }
    }
    if j >= sig.len() || sig[j].kind != TokenKind::RBrace {
        return None;
    }
    Some(j + 1)
}

fn validate_func_main_subset(tokens: &[Token]) -> SyxResult<()> {
    let sig: Vec<&Token> = significant(tokens);
    if sig.is_empty() {
        return Err(SyxDiagnostic::error(
            "syx-cst: subset requires 1..N `[export] [access] [static|async] func …` and/or `[access?] class Ident { … }` and/or data|final class and/or `export {…}` (optionally interleaved with top-level simple stmts)",
        )
        .with_code("CST001"));
    }
    let mut i = 0usize;
    let mut funcs = 0usize;
    let mut classes = 0usize;
    let mut exports = 0usize;
    let mut extensions = 0usize;
    while i < sig.len() {
        // W900/W901: optional `@Ident[(…)]` before any top-level decl.
        if let Some(next) = take_attr_prefix(&sig, i) {
            i = next;
            continue;
        }
        // W901: `import Ident`.
        if let Some(next) = take_import_shell(&sig, i) {
            i = next;
            continue;
        }
        // W883: thin export { Ident[,…] } list (before export func shell).
        if let Some(next) = take_export_list_shell(&sig, i) {
            exports += 1;
            i = next;
            continue;
        }
        if let Some((body_start, body_end, next)) = take_func_shell(&sig, i) {
            let body = &sig[body_start..body_end];
            if !is_simple_stmts_body(body) {
                return Err(SyxDiagnostic::error(
                    "syx-cst: subset body must be trivia-only or print(<simple|call-simple>) / let|var <ident> [: type] = <simple|call-simple> / return [<simple|call-simple>] stmts / while-simple / if-simple (braced or brace-less; optional else / else-if; call|compare|unary-!|!!|!!!|!!!!|unary-Neg|--|---|----|-----|------|-------|--------|---------|----------|-----------|------------|-------------|--------------|---------------|----------------|-----------------|------------------|paren-group(nest≤3)|logic(left-assoc &&|||, paren|!|!!|!!!|!!!!|-|--|---|----|-----|------|-------|--------|---------|----------|-----------|------------|-------------|--------------|---------------|----------------|-----------------|------------------|compare|call|simple ops); no depth>2 call / ((((…)))) / !!!!! / ------------------- / if case / if let)",
                )
                .with_code("CST001"));
            }
            funcs += 1;
            i = next;
            continue;
        }
        // W881/W882: thin data class / final class (optional generics; data where).
        if let Some(next) = take_data_class_shell(&sig, i) {
            classes += 1;
            i = next;
            continue;
        }
        if let Some(next) = take_final_class_shell(&sig, i) {
            classes += 1;
            i = next;
            continue;
        }
        // W879/W880: thin class shell (inherit + init; no deinit).
        if let Some(next) = take_class_shell(&sig, i) {
            classes += 1;
            i = next;
            continue;
        }
        // W900: extension Ident { func… }
        if let Some(next) = take_extension_shell(&sig, i) {
            extensions += 1;
            i = next;
            continue;
        }
        // W815: top-level simple stmts may interleave with funcs (ahead/between/trail).
        if let Some(next) = take_simple_stmt(&sig, i) {
            i = next;
            continue;
        }
        return Err(SyxDiagnostic::error(
            "syx-cst: subset allows `[export] [access] [static|async] func name([params]) [-> Ret] { … }` and/or `export { [Ident [as Ident][,…]] }` and/or `[access?] class Ident [: Ident(, Ident)*]? { var Ident: Ident[?]|init(…)|deinit|func… }` and/or `[access?] data class Ident[<T>](…) [where …]` / `[access?] final class Ident[<T>] [where …] { variants }` and/or `@builtin(\"…\")` + `extension Ident { func… }` interleaved with top-level print/let/var/return[<simple|call-simple>]/while-simple/if-simple only (no depth>2 call / ((((…)))) / !!!!! / ------------------- / static+async stack / if case / if let / member-index compound)",
        )
        .with_code("CST001"));
    }
    if funcs + classes + exports + extensions == 0 {
        return Err(SyxDiagnostic::error(
            "syx-cst: subset requires 1..N `[export] [access] [static|async] func …` and/or `[access?] class Ident { … }` and/or data|final class and/or `export {…}` and/or `extension` (optionally interleaved with top-level simple stmts / `@builtin`)",
        )
        .with_code("CST001"));
    }
    Ok(())
}

/// Format CST by concatenating lexemes (byte-identical for scanned trivia stream).
pub fn format_cst(root: &CstRoot) -> String {
    let mut out = String::new();
    for tok in &root.green.tokens {
        out.push_str(&tok.lexeme);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_roundtrip_preserves_comments() {
        let src = "// ahead\nfunc main() {\n  // inside\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("// ahead"));
        assert!(out.contains("// inside"));
        assert!(out.contains("/* trail */"));
    }

    #[test]
    fn subset_rejects_body_statements() {
        // Non-subset (Neg) remains out of subset (W848 reject face).
        let file = SourceFile::new("t.sy", "func main() { while -------------------n { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("quint neg must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_print_int_body_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // inside\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse print body");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(1)"));
    }

    #[test]
    fn subset_print_ident_body_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // inside\n  print(x)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse print ident");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_print_string_body_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // inside\n  print(\"hi\")\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse print string");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(\"hi\")"));
    }

    #[test]
    fn subset_multi_print_body_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // a\n  print(1)\n  print(x)\n  print(\"hi\")\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse multi print");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(1)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("print(\"hi\")"));
    }

    #[test]
    fn subset_let_bind_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // bind\n  let x = 1\n  let s = \"hi\"\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse let bind");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = 1"));
        assert!(out.contains("let s = \"hi\""));
    }

    #[test]
    fn subset_let_then_print_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // bind\n  let x = 1\n  let s = \"hi\"\n  print(x)\n  print(s)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse let+print");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = 1"));
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_return_simple_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // ret\n  return 0\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse return");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("return 0"));
    }

    #[test]
    fn subset_double_simple_roundtrip() {
        // W793 / ADR-1137: Double ∈ is_simple_value for print/let/return.
        let src = "// ahead\nfunc main() {\n  // float\n  let x = 1.5\n  print(x)\n  print(2.0)\n  return 3.14\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse double simple");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = 1.5"));
        assert!(out.contains("print(2.0)"));
        assert!(out.contains("return 3.14"));
    }

    #[test]
    fn subset_bool_simple_roundtrip() {
        // W794 / ADR-1138: Bool ∈ is_simple_value for print/let/return.
        let src = "// ahead\nfunc main() {\n  // flag\n  let x = true\n  print(x)\n  print(false)\n  return true\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bool simple");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = true"));
        assert!(out.contains("print(false)"));
        assert!(out.contains("return true"));
    }

    #[test]
    fn subset_nil_simple_roundtrip() {
        // W795 / ADR-1139: Nil ∈ is_simple_value for print/let/return.
        let src = "// ahead\nfunc main() {\n  // empty\n  let x = nil\n  print(x)\n  print(nil)\n  return nil\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nil simple");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = nil"));
        assert!(out.contains("print(nil)"));
        assert!(out.contains("return nil"));
    }

    #[test]
    fn subset_var_simple_roundtrip() {
        // W796 / ADR-1140: var <ident> = <simple> interleaved with print/let/return.
        let src = "// ahead\nfunc main() {\n  // mut\n  var x = 1\n  let y = true\n  print(x)\n  return nil\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse var simple");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("var x = 1"));
        assert!(out.contains("let y = true"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return nil"));
    }

    #[test]
    fn subset_var_call_rhs_roundtrip() {
        // W834 / ADR-1178: var x = call-simple.
        let src = "// ahead\nfunc main() {\n  // call rhs\n  var x = foo()\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse var call RHS");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("var x = foo()"));
    }

    #[test]
    fn subset_let_print_return_roundtrip() {
        let src = "// ahead\nfunc main() {\n  // bind\n  let x = 1\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse let+print+return");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = 1"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_rejects_non_print_stmt_in_multi() {
        let file = SourceFile::new("t.sy", "func main() { while -------------------n { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("quint neg must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_print_call_arg_roundtrip() {
        // W834 / ADR-1178: print(call-simple).
        let src = "// ahead\nfunc main() {\n  // print call\n  print(foo())\n  print(g(1))\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse print call arg");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(foo())"));
        assert!(out.contains("print(g(1))"));
    }

    #[test]
    fn subset_let_call_rhs_roundtrip() {
        // W834 / ADR-1178: let x = call-simple.
        let src = "// ahead\nfunc main() {\n  // let call\n  let x = foo()\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse let call RHS");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = foo()"));
    }

    #[test]
    fn subset_bare_return_roundtrip() {
        // W797 / ADR-1141: bare return[+print/var] with trivia.
        let src = "// ahead\nfunc main() {\n  // done\n  var x = 1\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bare return");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("var x = 1"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return"));
    }

    #[test]
    fn subset_bare_return_semi_roundtrip() {
        let src = "func main() { return; }\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bare return;");
        let out = format_cst(&root);
        assert_eq!(out, src);
    }

    #[test]
    fn subset_return_call_rhs_roundtrip() {
        // W834 / ADR-1178: return call-simple.
        let src = "// ahead\nfunc main() {\n  // ret call\n  return foo()\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse return call RHS");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("return foo()"));
    }

    #[test]
    fn subset_rejects_nested_call_as_arg() {
        // Was nested-call; reject face is Neg (W848).
        let file = SourceFile::new("t.sy", "func main() { while -------------------n { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("quint neg must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_multi_func_roundtrip() {
        // W798 / ADR-1142: 1..N zero-arg funcs with trivia.
        let src = "// ahead\nfunc helper() {\n  // a\n  print(1)\n}\nfunc main() {\n  // b\n  var x = 1\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse multi func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func helper"));
        assert!(out.contains("func main"));
        assert!(out.contains("print(1)"));
        assert!(out.contains("return"));
    }

    #[test]
    fn subset_three_funcs_roundtrip() {
        let src = "func a() {}\nfunc b() { print(1) }\nfunc c() { return }\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse three funcs");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_param_func_roundtrip() {
        // W816 / ADR-1160: bare Ident params (empty () still OK).
        let src = "// ahead\nfunc id(x) {\n  // param\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse param func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_func_roundtrip() {
        // W817 / ADR-1161: Ident[: Ident] params.
        let src = "// ahead\nfunc id(x: Int) {\n  // typed param\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_optional_func_roundtrip() {
        // W818 / ADR-1162: Ident[: Ident[?]] params.
        let src = "// ahead\nfunc id(x: Int?) {\n  // typed param optional\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param optional func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int?)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_amp_func_roundtrip() {
        // W819 / ADR-1163: Ident[: &Ident] params.
        let src = "// ahead\nfunc id(x: &Int) {\n  // typed param amp\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param amp func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: &Int)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_mut_amp_func_roundtrip() {
        // W820 / ADR-1164: Ident[: &mut Ident] params.
        let src = "// ahead\nfunc id(x: &mut Int) {\n  // typed param mut amp\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param mut amp func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: &mut Int)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_lifetime_amp_func_roundtrip() {
        // W821 / ADR-1165: Ident[: &'a Ident] params.
        let src = "// ahead\nfunc id(x: &'a Int) {\n  // typed param lifetime amp\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param lifetime amp func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: &'a Int)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_amp_optional_func_roundtrip() {
        // W822 / ADR-1166: Ident[: &Ident?] params.
        let src = "// ahead\nfunc id(x: &Int?) {\n  // typed param amp optional\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param amp optional func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: &Int?)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_generic_func_roundtrip() {
        // W823 / ADR-1167: Ident[: Ident<Ident>] params.
        let src = "// ahead\nfunc id(x: Foo<Int>) {\n  // typed param generic\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param generic func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<Int>)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_nested_generic_func_roundtrip() {
        // W824 / ADR-1168: Ident[: Ident<NestedNamed>] params.
        let src = "// ahead\nfunc id(x: Foo<Bar<Int>>) {\n  // typed param nested generic\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param nested generic func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<Bar<Int>>)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_typed_param_multi_arg_generic_func_roundtrip() {
        // W825 / ADR-1169: Ident[: Ident<Named(, Named)*>] params.
        let src = "// ahead\nfunc id(x: Foo<A, B>) {\n  // typed param multi-arg generic\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param multi-arg generic func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<A, B>)"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_amp_as_generic_arg_typed_param_func_roundtrip() {
        // W833 / ADR-1177: Amp as generic arg (`x: Foo<&Int>`).
        let src = "// ahead\nfunc id(x: Foo<&Int>) {\n  // amp as generic arg\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse amp-as-generic-arg typed param");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<&Int>)"));
    }

    #[test]
    fn subset_function_as_generic_arg_typed_param_func_roundtrip() {
        // W834 / ADR-1178: Function as generic arg (`x: Foo<(Int)->Bool>`).
        let src = "// ahead\nfunc id(x: Foo<(Int)->Bool>) {\n  // function as generic arg\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse Function-as-generic-arg typed param");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<(Int)->Bool>)"));
    }

    #[test]
    fn subset_func_return_roundtrip() {
        // W826 / ADR-1170: ) [-> Ident[?]] {.
        let src = "// ahead\nfunc id(x: Int) -> Int {\n  // func return\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Int"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_amp_roundtrip() {
        // W827 / ADR-1171: ) [-> &Ident] {.
        let src = "// ahead\nfunc id(x: Int) -> &Int {\n  // func return amp\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return amp");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &Int"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_mut_amp_roundtrip() {
        // W828 / ADR-1172: ) [-> &mut Ident] {.
        let src = "// ahead\nfunc id(x: Int) -> &mut Int {\n  // func return mut amp\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return mut amp");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &mut Int"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_lifetime_amp_roundtrip() {
        // W829 / ADR-1173: ) [-> &'a Ident] {.
        let src = "// ahead\nfunc id(x: Int) -> &'a Int {\n  // func return lifetime amp\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return lifetime amp");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &'a Int"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_amp_optional_roundtrip() {
        // W830 / ADR-1174: ) [-> &Ident?] {.
        let src = "// ahead\nfunc id(x: Int) -> &Int? {\n  // func return amp optional\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return amp optional");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &Int?"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_generic_roundtrip() {
        // W831 / ADR-1175: ) [-> Ident[<Ident>][?]] {.
        let src = "// ahead\nfunc id(x: Int) -> Foo<Int> {\n  // func return generic\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return generic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Foo<Int>"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_nested_generic_roundtrip() {
        // W832 / ADR-1176: ) [-> Ident[<NestedNamed>][?]] {.
        let src = "// ahead\nfunc id(x: Int) -> Foo<Bar<Int>> {\n  // func return nested generic\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return nested generic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Foo<Bar<Int>>"));
        assert!(out.contains("print(x)"));
        assert!(out.contains("return x"));
    }

    #[test]
    fn subset_func_return_multi_arg_generic_roundtrip() {
        // W833 / ADR-1177: ) [-> Ident[<Arg(, Arg)*>][?]] {.
        let src = "// ahead\nfunc id(x: Int) -> Foo<A, B> {\n  // func return multi-arg generic\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return multi-arg generic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Foo<A, B>"));
    }

    #[test]
    fn subset_func_return_function_roundtrip() {
        // W833 / ADR-1177: ) [-> Function] {.
        let src = "// ahead\nfunc id(x: Int) -> (Int) -> Bool {\n  // func return function\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return function");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> (Int) -> Bool"));
    }

    #[test]
    fn subset_toplevel_simple_between_funcs_roundtrip() {
        // W815 / ADR-1159: top-level simple stmts may sit between zero-arg funcs.
        let src = "func a() {}\nprint(1)\nfunc b() {}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse toplevel between");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_rejects_stmt_only_no_func() {
        let file = SourceFile::new("t.sy", "print(1)\nlet x = 2\n");
        let err = parse_cst_subset(&file).expect_err("stmt-only must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_typed_bind_roundtrip() {
        // W799 / ADR-1143: let|var Ident : Ident = simple with trivia.
        let src = "// ahead\nfunc main() {\n  // typed\n  let x: Int = 1\n  var s: String = \"hi\"\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed bind");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x: Int = 1"));
        assert!(out.contains("var s: String = \"hi\""));
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_typed_bind_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: Int = 1\n}\nfunc main() {\n  var x: Int = 2\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_optional_roundtrip() {
        // W800 / ADR-1144: let|var Ident : Ident? = simple with trivia.
        let src = "// ahead\nfunc main() {\n  // optional type\n  let x: Int? = nil\n  var s: String? = \"hi\"\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed optional");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x: Int? = nil"));
        assert!(out.contains("var s: String? = \"hi\""));
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_typed_optional_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: Int? = nil\n}\nfunc main() {\n  var x: Int? = 2\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed optional multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_reference_roundtrip() {
        // W801 / ADR-1145: let|var Ident : &Ident = simple with trivia.
        let src = "// ahead\nfunc main() {\n  // ref type\n  let x: &Int = y\n  var s: &String = t\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed reference");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x: &Int = y"));
        assert!(out.contains("var s: &String = t"));
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_typed_reference_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: &Int = y\n}\nfunc main() {\n  var x: &Int = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed reference multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_mut_reference_roundtrip() {
        // W802 / ADR-1146: let|var Ident : &mut Ident = simple with trivia.
        let src = "// ahead\nfunc main() {\n  // mut ref\n  let x: &mut Int = y\n  var s: &mut String = t\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed mut reference");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x: &mut Int = y"));
        assert!(out.contains("var s: &mut String = t"));
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_typed_mut_reference_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: &mut Int = y\n}\nfunc main() {\n  var x: &mut Int = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed mut reference multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_lifetime_reference_roundtrip() {
        // W803 / ADR-1147: let|var Ident : &'a Ident / &'a mut Ident = simple with trivia.
        let src = "// ahead\nfunc main() {\n  // lifetime ref\n  let x: &'a Int = y\n  var s: &'a mut String = t\n  print(x)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed lifetime reference");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x: &'a Int = y"));
        assert!(out.contains("var s: &'a mut String = t"));
        assert!(out.contains("print(x)"));
    }

    #[test]
    fn subset_typed_lifetime_reference_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: &'a Int = y\n}\nfunc main() {\n  var x: &'b mut Int = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed lifetime reference multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_function_type_roundtrip() {
        // W804 / ADR-1148: let|var Ident : (Ident…)[-> Ident] / () = simple with trivia.
        let src = "// ahead\nfunc main() {\n  // func type\n  let f: (Int) -> Int = id\n  var g: (Int, String) -> Bool = h\n  let u: () -> Void = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function type");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: (Int) -> Int = id"));
        assert!(out.contains("var g: (Int, String) -> Bool = h"));
        assert!(out.contains("let u: () -> Void = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_type_unit_and_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: () = unit\n}\nfunc main() {\n  var x: (Int) -> Int = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function type multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_function_param_reference_roundtrip() {
        // W805 / ADR-1149: Function params may be Amp Reference family.
        let src = "// ahead\nfunc main() {\n  // func param ref\n  let f: (&Int) -> Int = id\n  var g: (Int, &mut String) -> Bool = h\n  let u: (&'a Int) -> Void = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function param reference");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: (&Int) -> Int = id"));
        assert!(out.contains("var g: (Int, &mut String) -> Bool = h"));
        assert!(out.contains("let u: (&'a Int) -> Void = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_param_reference_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: (&mut Int) -> Int = id\n}\nfunc main() {\n  var x: (&'b mut String) -> Bool = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function param reference multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_function_param_optional_roundtrip() {
        // W806 / ADR-1150: Function params may be Optional Named (`Ident?`).
        let src = "// ahead\nfunc main() {\n  // func param optional\n  let f: (Int?) -> Int = id\n  var g: (String?, &mut Int) -> Bool = h\n  let u: (Int?, String?) -> Void = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function param optional");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: (Int?) -> Int = id"));
        assert!(out.contains("var g: (String?, &mut Int) -> Bool = h"));
        assert!(out.contains("let u: (Int?, String?) -> Void = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_param_optional_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: (Int?) -> Int = id\n}\nfunc main() {\n  var x: (String?, Bool?) -> Void = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function param optional multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_function_param_nested_roundtrip() {
        // W809 / ADR-1153: Function params may be nested Function TypeExpr.
        let src = "// ahead\nfunc main() {\n  // nested function param\n  let f: ((Int) -> Int) -> Int = id\n  var g: ((String?) -> String?) -> Void = h\n  let u: (() -> Void) -> Int? = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function param nested");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: ((Int) -> Int) -> Int = id"));
        assert!(out.contains("var g: ((String?) -> String?) -> Void = h"));
        assert!(out.contains("let u: (() -> Void) -> Int? = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_param_nested_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: ((Int) -> Int) -> Int = id\n}\nfunc main() {\n  var x: (() -> Bool?) -> Void = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function param nested multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_amp_optional_roundtrip() {
        // W811 / ADR-1155: Amp Reference may hold trailing `?` (bind / Function param / return).
        let src = "// ahead\nfunc main() {\n  // amp optional\n  let a: &Int? = p\n  var b: &mut String? = q\n  let f: (&Int?) -> Int = id\n  var g: (Int) -> &mut String? = h\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed amp optional");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let a: &Int? = p"));
        assert!(out.contains("var b: &mut String? = q"));
        assert!(out.contains("let f: (&Int?) -> Int = id"));
        assert!(out.contains("var g: (Int) -> &mut String? = h"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_amp_optional_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: &'a Int? = p\n}\nfunc main() {\n  var x: (Int) -> &'b mut String? = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed amp optional multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_generic_roundtrip() {
        // W812 / ADR-1156: Named may hold single-arg `<Ident>` (bind / Function param / return).
        let src = "// ahead\nfunc main() {\n  // typed generic\n  let a: Foo<Int> = p\n  var b: Bar<String>? = q\n  let f: (Foo<Int>) -> Int = id\n  var g: (Int) -> Bar<String> = h\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed generic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let a: Foo<Int> = p"));
        assert!(out.contains("var b: Bar<String>? = q"));
        assert!(out.contains("let f: (Foo<Int>) -> Int = id"));
        assert!(out.contains("var g: (Int) -> Bar<String> = h"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_generic_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: Foo<Int> = p\n}\nfunc main() {\n  var x: (Bar<String>?) -> Baz<Int> = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed generic multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_nested_generic_roundtrip() {
        // W813 / ADR-1157: Named generic arg may nest Named (bind / Function param / return).
        let src = "// ahead\nfunc main() {\n  // nested generic\n  let a: Foo<Bar<Int>> = p\n  var b: Box<Opt<String>>? = q\n  let f: (Foo<Bar<Int>>) -> Int = id\n  var g: (Int) -> Box<List<String>> = h\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed nested generic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let a: Foo<Bar<Int>> = p"));
        assert!(out.contains("var b: Box<Opt<String>>? = q"));
        assert!(out.contains("let f: (Foo<Bar<Int>>) -> Int = id"));
        assert!(out.contains("var g: (Int) -> Box<List<String>> = h"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_nested_generic_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: Foo<Bar<Int>> = p\n}\nfunc main() {\n  var x: (Box<Opt<String>>?) -> List<Int> = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed nested generic multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_multi_arg_generic_roundtrip() {
        // W814 / ADR-1158: Named generic may hold multiple Named args (bind / Function param / return).
        let src = "// ahead\nfunc main() {\n  // multi-arg generic\n  let a: Foo<A, B> = p\n  var b: Dict<String, Int>? = q\n  let f: (Foo<A, B>) -> Int = id\n  var g: (Int) -> Dict<String, List<Int>> = h\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed multi-arg generic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let a: Foo<A, B> = p"));
        assert!(out.contains("var b: Dict<String, Int>? = q"));
        assert!(out.contains("let f: (Foo<A, B>) -> Int = id"));
        assert!(out.contains("var g: (Int) -> Dict<String, List<Int>> = h"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_multi_arg_generic_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: Foo<A, B> = p\n}\nfunc main() {\n  var x: (Dict<String, Int>?) -> List<Int> = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed multi-arg generic multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_toplevel_simple_roundtrip() {
        // W815 / ADR-1159: ahead/between/trail simple + zero-arg funcs + typed multi-arg.
        let src = "// ahead\nprint(0)\nfunc helper() {\n  let n: Int = 1\n}\n// between\nlet x: Foo<A, B> = p\nvar y: Dict<String, Int>? = q\nprint(x)\nfunc main() {\n  print(y)\n  return\n}\nreturn\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse toplevel simple");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(0)"));
        assert!(out.contains("let x: Foo<A, B> = p"));
        assert!(out.contains("var y: Dict<String, Int>? = q"));
        assert!(out.contains("return\n/* trail */") || out.contains("return"));
    }

    #[test]
    fn subset_toplevel_simple_multi_func_roundtrip() {
        let src = "let a: Foo<A, B> = p\nfunc helper() {\n  print(a)\n}\nvar b: Dict<String, Int>? = q\nfunc main() {\n  print(b)\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse toplevel simple multi");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_param_func_multi_and_toplevel_roundtrip() {
        // W816 / ADR-1160: multi-param + toplevel simple interleaved.
        let src = "// ahead\nfunc id(x) {\n  print(x)\n  return x\n}\nfunc add(a, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse param func multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func add(a, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_multi_mixed_and_toplevel_roundtrip() {
        // W817 / ADR-1161: multi typed + mixed bare/typed + toplevel interleaved.
        let src = "// ahead\nfunc id(x: Int) {\n  print(x)\n  return x\n}\nfunc add(a: Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func add(a: Int, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_optional_multi_mixed_and_toplevel_roundtrip() {
        // W818 / ADR-1162: multi optional + mixed bare/typed/optional + toplevel interleaved.
        let src = "// ahead\nfunc id(x: Int?) {\n  print(x)\n  return x\n}\nfunc add(a: Int?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param optional multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func add(a: Int?, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_amp_multi_mixed_and_toplevel_roundtrip() {
        // W819 / ADR-1163: multi amp + mixed bare/Named/optional/amp + toplevel interleaved.
        let src = "// ahead\nfunc id(x: &Int) {\n  print(x)\n  return x\n}\nfunc add(a: &Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param amp multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func add(a: &Int, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_mut_amp_multi_mixed_and_toplevel_roundtrip() {
        // W820 / ADR-1164: multi mut-amp + mixed bare/Named/optional/amp/mut-amp + toplevel interleaved.
        let src = "// ahead\nfunc id(x: &mut Int) {\n  print(x)\n  return x\n}\nfunc add(a: &mut Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param mut amp multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func add(a: &mut Int, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_lifetime_amp_multi_mixed_and_toplevel_roundtrip() {
        // W821 / ADR-1165: multi lifetime ± mut + mixed bare/Named/optional/amp/mut-amp/lifetime + toplevel.
        let src = "// ahead\nfunc id(x: &'a Int) {\n  print(x)\n  return x\n}\nfunc add(a: &'a mut Int, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param lifetime amp multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: &'a Int)"));
        assert!(out.contains("func add(a: &'a mut Int, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_amp_optional_multi_mixed_and_toplevel_roundtrip() {
        // W822 / ADR-1166: multi Amp[?] ± mut ± lifetime + mixed + toplevel.
        let src = "// ahead\nfunc id(x: &Int?) {\n  print(x)\n  return x\n}\nfunc add(a: &mut Int?, b: &'a Int?) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param amp optional multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: &Int?)"));
        assert!(out.contains("func add(a: &mut Int?, b: &'a Int?)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_generic_multi_mixed_and_toplevel_roundtrip() {
        // W823 / ADR-1167: multi generic ± optional + mixed Amp/Named + toplevel.
        let src = "// ahead\nfunc id(x: Foo<Int>) {\n  print(x)\n  return x\n}\nfunc add(a: List<Int>?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param generic multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<Int>)"));
        assert!(out.contains("func add(a: List<Int>?, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_nested_generic_multi_mixed_and_toplevel_roundtrip() {
        // W824 / ADR-1168: multi nested ± optional + mixed Amp/Named + toplevel.
        let src = "// ahead\nfunc id(x: Foo<Bar<Int>>) {\n  print(x)\n  return x\n}\nfunc add(a: Box<List<Int>>?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param nested generic multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<Bar<Int>>)"));
        assert!(out.contains("func add(a: Box<List<Int>>?, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_typed_param_multi_arg_generic_multi_mixed_and_toplevel_roundtrip() {
        // W825 / ADR-1169: multi multi-arg ± optional + mixed Amp/Named + toplevel.
        let src = "// ahead\nfunc id(x: Foo<A, B>) {\n  print(x)\n  return x\n}\nfunc add(a: Dict<String, Int>?, b) {\n  let s = a\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed param multi-arg generic multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<A, B>)"));
        assert!(out.contains("func add(a: Dict<String, Int>?, b)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_amp_as_generic_arg_multi_mixed_and_toplevel_roundtrip() {
        // W833 / ADR-1177: Amp-as-generic-arg param + return + toplevel.
        let src = "// ahead\nfunc id(x: Foo<&Int>) {\n  print(x)\n  return x\n}\nfunc ret() -> Bar<&T> {\n  return\n}\nfunc mutp(x: Box<&mut String>) {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse amp-as-generic-arg multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<&Int>)"));
        assert!(out.contains("func ret() -> Bar<&T>"));
        assert!(out.contains("func mutp(x: Box<&mut String>)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_function_as_generic_arg_multi_mixed_and_toplevel_roundtrip() {
        // W834 / ADR-1178: Function-as-generic-arg param + return + toplevel.
        let src = "// ahead\nfunc id(x: Foo<(Int)->Bool>) {\n  print(x)\n  return x\n}\nfunc ret() -> Bar<()> {\n  return\n}\nfunc nest(x: Box<(Int)->((Int)->Bool)>) {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse Function-as-generic-arg multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Foo<(Int)->Bool>)"));
        assert!(out.contains("func ret() -> Bar<()>"));
        assert!(out.contains("func nest(x: Box<(Int)->((Int)->Bool)>)"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_rejects_depth_gt1_after_function_as_generic_arg() {
        // Triple bang remains CST001 (dry-run primary reject face).
        let file = SourceFile::new("t.sy", "func main() { while -------------------n { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("quint neg must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_func_return_multi_mixed_and_toplevel_roundtrip() {
        // W826 / ADR-1170: -> Ident / -> Ident? + params + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> Int {\n  print(x)\n  return x\n}\nfunc none() -> Void? {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Int"));
        assert!(out.contains("func none() -> Void?"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_amp_multi_mixed_and_toplevel_roundtrip() {
        // W827 / ADR-1171: -> &Ident + -> Ident[?] + params + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> &Int {\n  print(x)\n  return x\n}\nfunc bare() -> &String {\n  return\n}\nfunc none() -> Void? {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return amp multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &Int"));
        assert!(out.contains("func bare() -> &String"));
        assert!(out.contains("func none() -> Void?"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_mut_amp_multi_mixed_and_toplevel_roundtrip() {
        // W828 / ADR-1172: -> &mut Ident + -> &Ident + -> Ident[?] + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> &mut Int {\n  print(x)\n  return x\n}\nfunc bare() -> &mut String {\n  return\n}\nfunc amp() -> &Int {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return mut amp multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &mut Int"));
        assert!(out.contains("func bare() -> &mut String"));
        assert!(out.contains("func amp() -> &Int"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_lifetime_amp_multi_mixed_and_toplevel_roundtrip() {
        // W829 / ADR-1173: -> &'a Ident + -> &'a mut Ident + -> &mut Ident + -> &Ident + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> &'a Int {\n  print(x)\n  return x\n}\nfunc mut_life() -> &'b mut String {\n  return\n}\nfunc amp() -> &Int {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return lifetime amp multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &'a Int"));
        assert!(out.contains("func mut_life() -> &'b mut String"));
        assert!(out.contains("func amp() -> &Int"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_amp_optional_multi_mixed_and_toplevel_roundtrip() {
        // W830 / ADR-1174: -> &Ident? + -> &mut Ident? + -> &'a Ident? + -> &'a mut Ident? + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> &Int? {\n  print(x)\n  return x\n}\nfunc mut_opt() -> &mut String? {\n  return\n}\nfunc life() -> &'a Int? {\n  return\n}\nfunc mut_life() -> &'b mut String? {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return amp optional multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> &Int?"));
        assert!(out.contains("func mut_opt() -> &mut String?"));
        assert!(out.contains("func life() -> &'a Int?"));
        assert!(out.contains("func mut_life() -> &'b mut String?"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_generic_multi_mixed_and_toplevel_roundtrip() {
        // W831 / ADR-1175: -> Foo<Int> + -> Bar<String>? + Amp[?] + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> Foo<Int> {\n  print(x)\n  return x\n}\nfunc opt() -> Bar<String>? {\n  return\n}\nfunc amp() -> &Int? {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return generic multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Foo<Int>"));
        assert!(out.contains("func opt() -> Bar<String>?"));
        assert!(out.contains("func amp() -> &Int?"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_nested_generic_multi_mixed_and_toplevel_roundtrip() {
        // W832 / ADR-1176: -> Foo<Bar<Int>> + -> Box<List<String>>? + flat Foo<Int> + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> Foo<Bar<Int>> {\n  print(x)\n  return x\n}\nfunc opt() -> Box<List<String>>? {\n  return\n}\nfunc flat() -> Foo<Int> {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return nested generic multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Foo<Bar<Int>>"));
        assert!(out.contains("func opt() -> Box<List<String>>?"));
        assert!(out.contains("func flat() -> Foo<Int>"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_multi_arg_generic_multi_mixed_and_toplevel_roundtrip() {
        // W833 / ADR-1177: -> Foo<A, B> + -> Dict<String, Int>? + nested + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> Foo<A, B> {\n  print(x)\n  return x\n}\nfunc opt() -> Dict<String, Int>? {\n  return\n}\nfunc nested() -> Foo<Bar<Int>> {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return multi-arg generic multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> Foo<A, B>"));
        assert!(out.contains("func opt() -> Dict<String, Int>?"));
        assert!(out.contains("func nested() -> Foo<Bar<Int>>"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_func_return_function_multi_mixed_and_toplevel_roundtrip() {
        // W833 / ADR-1177: -> (Int) -> Bool + -> () + -> Ident + toplevel.
        let src = "// ahead\nfunc id(x: Int) -> (Int) -> Bool {\n  print(x)\n  return x\n}\nfunc unit() -> () {\n  return\n}\nfunc named() -> Int {\n  return\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse func return function multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("func id(x: Int) -> (Int) -> Bool"));
        assert!(out.contains("func unit() -> ()"));
        assert!(out.contains("func named() -> Int"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_call_rhs_multi_mixed_and_toplevel_roundtrip() {
        // W834 / ADR-1178: let/var/return call-simple + print call + toplevel.
        let src = "// ahead\nfunc main() {\n  let x = foo()\n  var y = g(1)\n  print(h(x, y))\n  return k()\n}\nprint(0)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse call rhs multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = foo()"));
        assert!(out.contains("var y = g(1)"));
        assert!(out.contains("print(h(x, y))"));
        assert!(out.contains("return k()"));
        assert!(out.contains("print(0)"));
    }

    #[test]
    fn subset_rejects_depth_gt1_after_call_simple() {
        // Triple bang remains CST001 this wave.
        let file = SourceFile::new("t.sy", "func main() { while -------------------n { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("quint neg must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_while_ident_cond_roundtrip() {
        // W835 / ADR-1179: while true / while flag + print/let body.
        let src = "// ahead\nfunc main() {\n  while true {\n    print(1)\n  }\n  while flag {\n    let x = 1\n    print(x)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse while ident cond");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while true"));
        assert!(out.contains("while flag"));
    }

    #[test]
    fn subset_while_call_cond_roundtrip() {
        // W835: while ready() / while tick(1).
        let src = "// ahead\nfunc main() {\n  while ready() {\n    print(1)\n  }\n  while tick(1) {\n    var y = 2\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse while call cond");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ready()"));
        assert!(out.contains("while tick(1)"));
    }

    #[test]
    fn subset_while_mixed_and_toplevel_roundtrip() {
        // W835: mixed body + toplevel while.
        let src = "// ahead\nwhile go {\n  print(0)\n}\nfunc main() {\n  while true {\n    print(1)\n    return\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse while mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while go"));
        assert!(out.contains("return"));
    }

    #[test]
    fn subset_while_nested_roundtrip() {
        // W836 / ADR-1180: nested while inside while body.
        let src = "// ahead\nfunc main() {\n  while true {\n    while flag {\n      print(1)\n    }\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested while");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while true"));
        assert!(out.contains("while flag"));
    }

    #[test]
    fn subset_while_compare_cond_roundtrip() {
        // W836: compare cond `<`/`>`/`<=`/`>=`.
        let src = "// ahead\nfunc main() {\n  while x < n {\n    print(x)\n  }\n  while a >= b {\n    print(a)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse while compare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while x < n"));
        assert!(out.contains("while a >= b"));
    }

    #[test]
    fn subset_while_deeper_mixed_roundtrip() {
        // W836: nested × compare + toplevel.
        let src = "// ahead\nwhile go {\n  while i < n {\n    print(i)\n  }\n}\nfunc main() {\n  while ready() {\n    while flag {\n      print(1)\n    }\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse while deeper mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while i < n"));
        assert!(out.contains("while ready()"));
    }

    #[test]
    fn subset_rejects_while_triple_bang() {
        // W870: Bang depth>4 (`!!!!!`) rejected.
        let file = SourceFile::new("t.sy", "func main() { while !!!!!flag { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("quint bang must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_while_double_neg() {
        // W869: Neg depth>18 (`-------------------`) rejected.
        let file = SourceFile::new("t.sy", "func main() { while -------------------n { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("octodec neg must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_while_nested_paren() {
        // W871: paren nest depth>3 (`((((a))))`) rejected (cap frozen at 3).
        let file = SourceFile::new("t.sy", "func main() { while ((((a)))) { print(1) } }\n");
        let err = parse_cst_subset(&file).expect_err("paren nest>3 must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_while_braceless_nested() {
        // W843: brace-less body may not be nested while.
        let file = SourceFile::new("t.sy", "func main() { while true while flag print(1) }\n");
        let err = parse_cst_subset(&file).expect_err("braceless nested while must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }



    #[test]
    fn subset_nested_call_print_roundtrip() {
        // W838 / ADR-1182: print(foo(bar())) depth≤1.
        let src = "// ahead\nfunc main() {\n  print(foo(bar()))\n  print(g(h(1)))\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested call print");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(foo(bar()))"));
        assert!(out.contains("print(g(h(1)))"));
    }

    #[test]
    fn subset_nested_call_rhs_roundtrip() {
        // W838: let/var/return nested call RHS.
        let src = "// ahead\nfunc main() {\n  let x = foo(bar())\n  var y = g(h(1))\n  return f(k())\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested call rhs");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = foo(bar())"));
        assert!(out.contains("return f(k())"));
    }

    #[test]
    fn subset_nested_call_mixed_roundtrip() {
        // W838: nested call + while + effect.
        let src = "// ahead\nfunc go() requires Pure {\n  print(foo(bar()))\n}\nfunc main() {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested call mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(foo(bar()))"));
        assert!(out.contains("let x = f(g())"));
    }




    #[test]
    fn subset_while_logic_and_roundtrip() {
        // W844 / ADR-1188 face1: while a && b braced.
        let src = "// ahead\nfunc main() {\n  while a && b {\n    print(1)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic and");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while a && b"));
    }

    #[test]
    fn subset_while_logic_or_roundtrip() {
        // W844 face2: || braced + && brace-less.
        let src = "// ahead\nfunc main() {\n  while x || y {\n    print(1)\n  }\n  while a && b\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic or");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while x || y"));
        assert!(out.contains("while a && b"));
    }

    #[test]
    fn subset_while_logic_mixed_roundtrip() {
        // W844 face3: logic × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while a && b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while x || y\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while a && b"));
        assert!(out.contains("while x || y"));
    }

    #[test]
    fn subset_while_logic_chain_roundtrip() {
        // W845 / ADR-1189 face1: same-op chain.
        let src = "// ahead\nfunc main() {\n  while a && b && c {\n    print(1)\n  }\n  while x || y || z\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic chain");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while a && b && c"));
        assert!(out.contains("while x || y || z"));
    }

    #[test]
    fn subset_while_logic_cmp_roundtrip() {
        // W845 face2: compare×logic.
        let src = "// ahead\nfunc main() {\n  while a < b && c {\n    print(1)\n  }\n  while x >= n || flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic cmp");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while a < b && c"));
        assert!(out.contains("while x >= n || flag"));
    }

    #[test]
    fn subset_while_logic_deep_mixed_roundtrip() {
        // W845 face3: chain × cmp×logic × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while a && b && c {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while i < n || flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic deep mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while a && b && c"));
        assert!(out.contains("while i < n || flag"));
    }

    #[test]
    fn subset_while_logic_mixedops_roundtrip() {
        // W846 / ADR-1190 face1: mixed-ops left-assoc.
        let src = "// ahead\nfunc main() {\n  while a && b || c {\n    print(1)\n  }\n  while x || y && z\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic mixedops");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while a && b || c"));
        assert!(out.contains("while x || y && z"));
    }

    #[test]
    fn subset_while_logic_call_roundtrip() {
        // W846 face2: call×logic.
        let src = "// ahead\nfunc main() {\n  while ready() && flag {\n    print(1)\n  }\n  while tick(1) || done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic call");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ready() && flag"));
        assert!(out.contains("while tick(1) || done"));
    }

    #[test]
    fn subset_while_logic_final_mixed_roundtrip() {
        // W846 face3: mixed-ops × call×logic × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while i < n || tick(1)\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse logic final mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ready() && a || b"));
        assert!(out.contains("while i < n || tick(1)"));
    }

    #[test]
    fn subset_while_unary_bare_roundtrip() {
        // W847 / ADR-1191 face1: bare unary !.
        let src = "// ahead\nfunc main() {\n  while !flag {\n    print(1)\n  }\n  while !done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse unary bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !flag"));
        assert!(out.contains("while !done"));
    }

    #[test]
    fn subset_while_unary_logic_roundtrip() {
        // W847 face2: unary×logic.
        let src = "// ahead\nfunc main() {\n  while !a && b {\n    print(1)\n  }\n  while !ready() || done\n  print(1)\n  while !x && y || z\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse unary logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !a && b"));
        assert!(out.contains("while !ready() || done"));
        assert!(out.contains("while !x && y || z"));
    }

    #[test]
    fn subset_while_unary_mixed_roundtrip() {
        // W847 face3: unary × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse unary mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !ready() && a || b"));
        assert!(out.contains("while !flag"));
    }

    #[test]
    fn subset_while_paren_bare_roundtrip() {
        // W848 / ADR-1192 face1: bare paren / paren×logic.
        let src = "// ahead\nfunc main() {\n  while (a && b) || c {\n    print(1)\n  }\n  while a && (b || c)\n  print(1)\n  while (flag)\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while (a && b) || c"));
        assert!(out.contains("while a && (b || c)"));
        assert!(out.contains("while (flag)"));
    }

    #[test]
    fn subset_while_paren_unary_roundtrip() {
        // W848 face2: paren×unary·call.
        let src = "// ahead\nfunc main() {\n  while !(a && b) {\n    print(1)\n  }\n  while (!ready() && flag)\n  print(1)\n  while (tick(1) || done)\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren unary");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !(a && b)"));
        assert!(out.contains("while (!ready() && flag)"));
        assert!(out.contains("while (tick(1) || done)"));
    }

    #[test]
    fn subset_while_paren_mixed_roundtrip() {
        // W848 face3: paren × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while (a && b) || ready() {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !(x || y)\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while (a && b) || ready()"));
        assert!(out.contains("while !(x || y)"));
    }

    #[test]
    fn subset_while_neg_bare_roundtrip() {
        // W849 / ADR-1193 face1: bare Neg `-`.
        let src = "// ahead\nfunc main() {\n  while -n {\n    print(1)\n  }\n  while -done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -n"));
        assert!(out.contains("while -done"));
    }

    #[test]
    fn subset_while_neg_logic_roundtrip() {
        // W849 face2: Neg×logic·call.
        let src = "// ahead\nfunc main() {\n  while -a && b {\n    print(1)\n  }\n  while -ready() || done\n  print(1)\n  while (-n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -a && b"));
        assert!(out.contains("while -ready() || done"));
        assert!(out.contains("while (-n) && flag"));
    }

    #[test]
    fn subset_while_neg_mixed_roundtrip() {
        // W849 face3: Neg × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -ready() && a || b"));
        assert!(out.contains("while -flag"));
    }

    #[test]
    fn subset_while_bang2_bare_roundtrip() {
        // W850 / ADR-1194 face1: bare stacked `!!`.
        let src = "// ahead\nfunc main() {\n  while !!flag {\n    print(1)\n  }\n  while !!done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang2 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!flag"));
        assert!(out.contains("while !!done"));
    }

    #[test]
    fn subset_while_bang2_logic_roundtrip() {
        // W850 face2: !!×logic·call.
        let src = "// ahead\nfunc main() {\n  while !!a && b {\n    print(1)\n  }\n  while !!ready() || done\n  print(1)\n  while (!!flag) && x\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang2 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!a && b"));
        assert!(out.contains("while !!ready() || done"));
        assert!(out.contains("while (!!flag) && x"));
    }

    #[test]
    fn subset_while_bang2_mixed_roundtrip() {
        // W850 face3: !! × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !!ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !!flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang2 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!ready() && a || b"));
        assert!(out.contains("while !!flag"));
    }

    #[test]
    fn subset_while_paren2_bare_roundtrip() {
        // W851 / ADR-1195 face1: bare nested paren depth=2.
        let src = "// ahead\nfunc main() {\n  while ((flag)) {\n    print(1)\n  }\n  while ((done))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren2 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ((flag))"));
        assert!(out.contains("while ((done))"));
    }

    #[test]
    fn subset_while_paren2_logic_roundtrip() {
        // W851 face2: nest×logic·call / bang nest.
        let src = "// ahead\nfunc main() {\n  while ((a && b)) || c {\n    print(1)\n  }\n  while ((tick(1)))\n  print(1)\n  while !(!(ready()))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren2 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ((a && b)) || c"));
        assert!(out.contains("while ((tick(1)))"));
        assert!(out.contains("while !(!(ready()))"));
    }

    #[test]
    fn subset_while_paren2_mixed_roundtrip() {
        // W851 face3: nest × braced/brace-less / effect / where / depth2 / !!·Neg.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ((!!ready())) && a || -b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ((flag))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren2 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ((!!ready())) && a || -b"));
        assert!(out.contains("while ((flag))"));
    }

    #[test]
    fn subset_while_paren3_bare_roundtrip() {
        // W871 / ADR-1215 face1: bare nested paren depth=3.
        let src = "// ahead\nfunc main() {\n  while (((flag))) {\n    print(1)\n  }\n  while (((done)))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren3 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while (((flag)))"));
        assert!(out.contains("while (((done)))"));
    }

    #[test]
    fn subset_while_paren3_logic_roundtrip() {
        // W871 face2: nest×logic·call / bang nest.
        let src = "// ahead\nfunc main() {\n  while (((a && b))) || c {\n    print(1)\n  }\n  while (((tick(1))))\n  print(1)\n  while !(((ready())))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren3 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while (((a && b))) || c"));
        assert!(out.contains("while (((tick(1))))"));
        assert!(out.contains("while !(((ready())))"));
    }

    #[test]
    fn subset_while_paren3_mixed_roundtrip() {
        // W871 face3: nest3 × braced/brace-less / effect / where / depth2 / !!!!·Neg.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while (((!!!!ready()))) && a || -b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while (((flag)))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse paren3 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while (((!!!!ready()))) && a || -b"));
        assert!(out.contains("while (((flag)))"));
    }

    #[test]
    fn subset_comment_line_roundtrip() {
        // W872 / ADR-1216 face1: file-level + body line comments.
        let src = "// ahead\nfunc main() {\n  // inside\n  print(1)\n  // mid\n  let x = 1\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse comment line");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("// ahead"));
        assert!(out.contains("// inside"));
        assert!(out.contains("// mid"));
        // Caps frozen: Neg≤18 / Bang≤4 / paren≤3.
        assert_eq!(PAREN_NEST_MAX, 3);
        let deep = SourceFile::new("d.sy", "func main() { while ((((a)))) { print(1) } }\n");
        assert_eq!(
            parse_cst_subset(&deep).expect_err("paren4").code.as_deref(),
            Some("CST001")
        );
        let bang5 = SourceFile::new("b.sy", "func main() { while !!!!!flag { print(1) } }\n");
        assert_eq!(
            parse_cst_subset(&bang5).expect_err("bang5").code.as_deref(),
            Some("CST001")
        );
        let neg19 =
            SourceFile::new("n.sy", "func main() { while -------------------n { print(1) } }\n");
        assert_eq!(
            parse_cst_subset(&neg19).expect_err("neg19").code.as_deref(),
            Some("CST001")
        );
    }

    #[test]
    fn subset_comment_stmt_roundtrip() {
        // W872 face2: line×stmt·while / inline block before paren nest.
        let src = "// ahead\nfunc main() {\n  let x = 1 // trail-let\n  while flag // trail-while\n  print(1)\n  while /*c*/ (((a)))\n  print(2)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse comment stmt");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = 1 // trail-let"));
        assert!(out.contains("while flag // trail-while"));
        assert!(out.contains("while /*c*/ (((a)))"));
    }

    #[test]
    fn subset_comment_mixed_roundtrip() {
        // W872 face3: // + /* */ × brace-less / effect / where / depth2 / !!!!·Neg.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  // body\n  while (((!!!!ready()))) && a || -b /* mid */\n  print(foo(bar(baz())))\n}\nfunc main() {\n  /* block */\n  while (((flag)))\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse comment mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("// body"));
        assert!(out.contains("/* mid */"));
        assert!(out.contains("/* block */"));
        assert!(out.contains("/* trail */"));
    }


    #[test]
    fn subset_while_bang3_bare_roundtrip() {
        // W852 / ADR-1196 face1: bare stacked `!!!`.
        let src = "// ahead\nfunc main() {\n  while !!!flag {\n    print(1)\n  }\n  while !!!done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang3 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!!flag"));
        assert!(out.contains("while !!!done"));
    }

    #[test]
    fn subset_while_bang3_logic_roundtrip() {
        // W852 face2: !!!×logic·call.
        let src = "// ahead\nfunc main() {\n  while !!!a && b {\n    print(1)\n  }\n  while !!!ready() || done\n  print(1)\n  while (!!!flag) && x\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang3 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!!a && b"));
        assert!(out.contains("while !!!ready() || done"));
        assert!(out.contains("while (!!!flag) && x"));
    }

    #[test]
    fn subset_while_bang3_mixed_roundtrip() {
        // W852 face3: !!! × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !!!ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !!!flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang3 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!!ready() && a || b"));
        assert!(out.contains("while !!!flag"));
    }

    #[test]
    fn subset_while_bang4_bare_roundtrip() {
        // W870 / ADR-1214 face1: bare stacked `!!!!`.
        let src = "// ahead\nfunc main() {\n  while !!!!flag {\n    print(1)\n  }\n  while !!!!done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang4 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!!!flag"));
        assert!(out.contains("while !!!!done"));
    }

    #[test]
    fn subset_while_bang4_logic_roundtrip() {
        // W870 face2: !!!!×logic·call.
        let src = "// ahead\nfunc main() {\n  while !!!!a && b {\n    print(1)\n  }\n  while !!!!ready() || done\n  print(1)\n  while (!!!!flag) && x\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang4 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!!!a && b"));
        assert!(out.contains("while !!!!ready() || done"));
        assert!(out.contains("while (!!!!flag) && x"));
    }

    #[test]
    fn subset_while_bang4_mixed_roundtrip() {
        // W870 face3: !!!! × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while !!!!ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while !!!!flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bang4 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while !!!!ready() && a || b"));
        assert!(out.contains("while !!!!flag"));
    }


    #[test]
    fn subset_while_neg2_bare_roundtrip() {
        // W853 / ADR-1197 face1: bare stacked `--`.
        let src = "// ahead\nfunc main() {\n  while --n {\n    print(1)\n  }\n  while --done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg2 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --n"));
        assert!(out.contains("while --done"));
    }

    #[test]
    fn subset_while_neg2_logic_roundtrip() {
        // W853 face2: --×logic·call.
        let src = "// ahead\nfunc main() {\n  while --a && b {\n    print(1)\n  }\n  while --ready() || done\n  print(1)\n  while (--n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg2 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --a && b"));
        assert!(out.contains("while --ready() || done"));
        assert!(out.contains("while (--n) && flag"));
    }

    #[test]
    fn subset_while_neg2_mixed_roundtrip() {
        // W853 face3: -- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while --ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while --n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg2 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --ready() && a || b"));
        assert!(out.contains("while --n"));
    }

    #[test]
    fn subset_while_neg3_bare_roundtrip() {
        // W854 / ADR-1198 face1: bare stacked `---`.
        let src = "// ahead\nfunc main() {\n  while ---n {\n    print(1)\n  }\n  while ---done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg3 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---n"));
        assert!(out.contains("while ---done"));
    }

    #[test]
    fn subset_while_neg3_logic_roundtrip() {
        // W854 face2: ---×logic·call.
        let src = "// ahead\nfunc main() {\n  while ---a && b {\n    print(1)\n  }\n  while ---ready() || done\n  print(1)\n  while (---n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg3 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---a && b"));
        assert!(out.contains("while ---ready() || done"));
        assert!(out.contains("while (---n) && flag"));
    }

    #[test]
    fn subset_while_neg3_mixed_roundtrip() {
        // W854 face3: --- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ---ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ---n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg3 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---ready() && a || b"));
        assert!(out.contains("while ---n"));
    }

    #[test]
    fn subset_while_neg4_bare_roundtrip() {
        // W855 / ADR-1199 face1: bare stacked `----`.
        let src = "// ahead\nfunc main() {\n  while ----n {\n    print(1)\n  }\n  while ----done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg4 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----n"));
        assert!(out.contains("while ----done"));
    }

    #[test]
    fn subset_while_neg4_logic_roundtrip() {
        // W855 face2: ----×logic·call.
        let src = "// ahead\nfunc main() {\n  while ----a && b {\n    print(1)\n  }\n  while ----ready() || done\n  print(1)\n  while (----n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg4 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----a && b"));
        assert!(out.contains("while ----ready() || done"));
        assert!(out.contains("while (----n) && flag"));
    }

    #[test]
    fn subset_while_neg4_mixed_roundtrip() {
        // W855 face3: ---- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ----ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ----n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg4 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----ready() && a || b"));
        assert!(out.contains("while ----n"));
    }

    #[test]
    fn subset_while_neg5_bare_roundtrip() {
        // W856 / ADR-1200 face1: bare stacked `-----`.
        let src = "// ahead\nfunc main() {\n  while -----n {\n    print(1)\n  }\n  while -----done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg5 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----n"));
        assert!(out.contains("while -----done"));
    }

    #[test]
    fn subset_while_neg5_logic_roundtrip() {
        // W856 face2: -----×logic·call.
        let src = "// ahead\nfunc main() {\n  while -----a && b {\n    print(1)\n  }\n  while -----ready() || done\n  print(1)\n  while (-----n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg5 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----a && b"));
        assert!(out.contains("while -----ready() || done"));
        assert!(out.contains("while (-----n) && flag"));
    }

    #[test]
    fn subset_while_neg5_mixed_roundtrip() {
        // W856 face3: ----- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -----ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -----n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg5 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----ready() && a || b"));
        assert!(out.contains("while -----n"));
    }

    #[test]
    fn subset_while_neg6_bare_roundtrip() {
        // W857 / ADR-1201 face1: bare stacked `------`.
        let src = "// ahead\nfunc main() {\n  while ------n {\n    print(1)\n  }\n  while ------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg6 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------n"));
        assert!(out.contains("while ------done"));
    }

    #[test]
    fn subset_while_neg6_logic_roundtrip() {
        // W857 face2: ------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ------a && b {\n    print(1)\n  }\n  while ------ready() || done\n  print(1)\n  while (------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg6 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------a && b"));
        assert!(out.contains("while ------ready() || done"));
        assert!(out.contains("while (------n) && flag"));
    }

    #[test]
    fn subset_while_neg6_mixed_roundtrip() {
        // W857 face3: ------ × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg6 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------ready() && a || b"));
        assert!(out.contains("while ------n"));
    }

    #[test]
    fn subset_while_neg7_bare_roundtrip() {
        // W858 / ADR-1202 face1: bare stacked `-------`.
        let src = "// ahead\nfunc main() {\n  while -------n {\n    print(1)\n  }\n  while -------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg7 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -------n"));
        assert!(out.contains("while -------done"));
    }

    #[test]
    fn subset_while_neg7_logic_roundtrip() {
        // W858 face2: -------×logic·call.
        let src = "// ahead\nfunc main() {\n  while -------a && b {\n    print(1)\n  }\n  while -------ready() || done\n  print(1)\n  while (-------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg7 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -------a && b"));
        assert!(out.contains("while -------ready() || done"));
        assert!(out.contains("while (-------n) && flag"));
    }

    #[test]
    fn subset_while_neg7_mixed_roundtrip() {
        // W858 face3: ------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg7 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -------ready() && a || b"));
        assert!(out.contains("while -------n"));
    }

    #[test]
    fn subset_while_neg8_bare_roundtrip() {
        // W859 / ADR-1203 face1: bare stacked `--------`.
        let src = "// ahead\nfunc main() {\n  while --------n {\n    print(1)\n  }\n  while --------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg8 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --------n"));
        assert!(out.contains("while --------done"));
    }

    #[test]
    fn subset_while_neg8_logic_roundtrip() {
        // W859 face2: --------×logic·call.
        let src = "// ahead\nfunc main() {\n  while --------a && b {\n    print(1)\n  }\n  while --------ready() || done\n  print(1)\n  while (--------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg8 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --------a && b"));
        assert!(out.contains("while --------ready() || done"));
        assert!(out.contains("while (--------n) && flag"));
    }

    #[test]
    fn subset_while_neg8_mixed_roundtrip() {
        // W859 face3: -------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while --------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while --------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg8 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --------ready() && a || b"));
        assert!(out.contains("while --------n"));
    }

    #[test]
    fn subset_while_neg9_bare_roundtrip() {
        // W860 / ADR-1204 face1: bare stacked `---------`.
        let src = "// ahead\nfunc main() {\n  while ---------n {\n    print(1)\n  }\n  while ---------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg9 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---------n"));
        assert!(out.contains("while ---------done"));
    }

    #[test]
    fn subset_while_neg9_logic_roundtrip() {
        // W860 face2: ---------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ---------a && b {\n    print(1)\n  }\n  while ---------ready() || done\n  print(1)\n  while (---------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg9 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---------a && b"));
        assert!(out.contains("while ---------ready() || done"));
        assert!(out.contains("while (---------n) && flag"));
    }

    #[test]
    fn subset_while_neg9_mixed_roundtrip() {
        // W860 face3: --------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ---------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ---------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg9 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---------ready() && a || b"));
        assert!(out.contains("while ---------n"));
    }

    #[test]
    fn subset_while_neg10_bare_roundtrip() {
        // W861 / ADR-1205 face1: bare stacked `----------`.
        let src = "// ahead\nfunc main() {\n  while ----------n {\n    print(1)\n  }\n  while ----------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg10 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----------n"));
        assert!(out.contains("while ----------done"));
    }

    #[test]
    fn subset_while_neg10_logic_roundtrip() {
        // W861 face2: ----------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ----------a && b {\n    print(1)\n  }\n  while ----------ready() || done\n  print(1)\n  while (----------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg10 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----------a && b"));
        assert!(out.contains("while ----------ready() || done"));
        assert!(out.contains("while (----------n) && flag"));
    }

    #[test]
    fn subset_while_neg10_mixed_roundtrip() {
        // W861 face3: ---------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ----------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ----------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg10 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----------ready() && a || b"));
        assert!(out.contains("while ----------n"));
    }

    #[test]
    fn subset_while_neg11_bare_roundtrip() {
        // W862 / ADR-1206 face1: bare stacked `-----------`.
        let src = "// ahead\nfunc main() {\n  while -----------n {\n    print(1)\n  }\n  while -----------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg11 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----------n"));
        assert!(out.contains("while -----------done"));
    }

    #[test]
    fn subset_while_neg11_logic_roundtrip() {
        // W862 face2: -----------×logic·call.
        let src = "// ahead\nfunc main() {\n  while -----------a && b {\n    print(1)\n  }\n  while -----------ready() || done\n  print(1)\n  while (-----------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg11 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----------a && b"));
        assert!(out.contains("while -----------ready() || done"));
        assert!(out.contains("while (-----------n) && flag"));
    }

    #[test]
    fn subset_while_neg11_mixed_roundtrip() {
        // W862 face3: ----------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -----------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -----------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg11 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----------ready() && a || b"));
        assert!(out.contains("while -----------n"));
    }

    #[test]
    fn subset_while_neg12_bare_roundtrip() {
        // W863 / ADR-1207 face1: bare stacked `------------`.
        let src = "// ahead\nfunc main() {\n  while ------------n {\n    print(1)\n  }\n  while ------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg12 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------------n"));
        assert!(out.contains("while ------------done"));
    }

    #[test]
    fn subset_while_neg12_logic_roundtrip() {
        // W863 face2: ------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ------------a && b {\n    print(1)\n  }\n  while ------------ready() || done\n  print(1)\n  while (------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg12 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------------a && b"));
        assert!(out.contains("while ------------ready() || done"));
        assert!(out.contains("while (------------n) && flag"));
    }

    #[test]
    fn subset_while_neg12_mixed_roundtrip() {
        // W863 face3: ------------ × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg12 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------------ready() && a || b"));
        assert!(out.contains("while ------------n"));
    }

    #[test]
    fn subset_while_neg13_bare_roundtrip() {
        // W864 / ADR-1208 face1: bare stacked `-------------`.
        let src = "// ahead\nfunc main() {\n  while -------------n {\n    print(1)\n  }\n  while -------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg13 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -------------n"));
        assert!(out.contains("while -------------done"));
    }

    #[test]
    fn subset_while_neg13_logic_roundtrip() {
        // W864 face2: -------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while -------------a && b {\n    print(1)\n  }\n  while -------------ready() || done\n  print(1)\n  while (-------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg13 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -------------a && b"));
        assert!(out.contains("while -------------ready() || done"));
        assert!(out.contains("while (-------------n) && flag"));
    }

    #[test]
    fn subset_while_neg13_mixed_roundtrip() {
        // W864 face3: ------------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg13 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -------------ready() && a || b"));
        assert!(out.contains("while -------------n"));
    }

    #[test]
    fn subset_while_neg14_bare_roundtrip() {
        // W865 / ADR-1209 face1: bare stacked `--------------`.
        let src = "// ahead\nfunc main() {\n  while --------------n {\n    print(1)\n  }\n  while --------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg14 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --------------n"));
        assert!(out.contains("while --------------done"));
    }

    #[test]
    fn subset_while_neg14_logic_roundtrip() {
        // W865 face2: --------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while --------------a && b {\n    print(1)\n  }\n  while --------------ready() || done\n  print(1)\n  while (--------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg14 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --------------a && b"));
        assert!(out.contains("while --------------ready() || done"));
        assert!(out.contains("while (--------------n) && flag"));
    }

    #[test]
    fn subset_while_neg14_mixed_roundtrip() {
        // W865 face3: -------------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while --------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while --------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg14 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while --------------ready() && a || b"));
        assert!(out.contains("while --------------n"));
    }

    #[test]
    fn subset_while_neg15_bare_roundtrip() {
        // W866 / ADR-1210 face1: bare stacked `---------------`.
        let src = "// ahead\nfunc main() {\n  while ---------------n {\n    print(1)\n  }\n  while ---------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg15 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---------------n"));
        assert!(out.contains("while ---------------done"));
    }

    #[test]
    fn subset_while_neg15_logic_roundtrip() {
        // W866 face2: ---------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ---------------a && b {\n    print(1)\n  }\n  while ---------------ready() || done\n  print(1)\n  while (---------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg15 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---------------a && b"));
        assert!(out.contains("while ---------------ready() || done"));
        assert!(out.contains("while (---------------n) && flag"));
    }

    #[test]
    fn subset_while_neg15_mixed_roundtrip() {
        // W866 face3: --------------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ---------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ---------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg15 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ---------------ready() && a || b"));
        assert!(out.contains("while ---------------n"));
    }

    #[test]
    fn subset_while_neg16_bare_roundtrip() {
        // W867 / ADR-1211 face1: bare stacked `----------------`.
        let src = "// ahead\nfunc main() {\n  while ----------------n {\n    print(1)\n  }\n  while ----------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg16 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----------------n"));
        assert!(out.contains("while ----------------done"));
    }

    #[test]
    fn subset_while_neg16_logic_roundtrip() {
        // W867 face2: ----------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ----------------a && b {\n    print(1)\n  }\n  while ----------------ready() || done\n  print(1)\n  while (----------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg16 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----------------a && b"));
        assert!(out.contains("while ----------------ready() || done"));
        assert!(out.contains("while (----------------n) && flag"));
    }

    #[test]
    fn subset_while_neg16_mixed_roundtrip() {
        // W867 face3: ---------------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ----------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ----------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg16 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ----------------ready() && a || b"));
        assert!(out.contains("while ----------------n"));
    }

    #[test]
    fn subset_while_neg17_bare_roundtrip() {
        // W868 / ADR-1212 face1: bare stacked `-----------------`.
        let src = "// ahead\nfunc main() {\n  while -----------------n {\n    print(1)\n  }\n  while -----------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg17 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----------------n"));
        assert!(out.contains("while -----------------done"));
    }

    #[test]
    fn subset_while_neg17_logic_roundtrip() {
        // W868 face2: -----------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while -----------------a && b {\n    print(1)\n  }\n  while -----------------ready() || done\n  print(1)\n  while (-----------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg17 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----------------a && b"));
        assert!(out.contains("while -----------------ready() || done"));
        assert!(out.contains("while (-----------------n) && flag"));
    }

    #[test]
    fn subset_while_neg17_mixed_roundtrip() {
        // W868 face3: ----------------- × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while -----------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while -----------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg17 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while -----------------ready() && a || b"));
        assert!(out.contains("while -----------------n"));
    }

    #[test]
    fn subset_while_neg18_bare_roundtrip() {
        // W869 / ADR-1213 face1: bare stacked `------------------`.
        let src = "// ahead\nfunc main() {\n  while ------------------n {\n    print(1)\n  }\n  while ------------------done\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg18 bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------------------n"));
        assert!(out.contains("while ------------------done"));
    }

    #[test]
    fn subset_while_neg18_logic_roundtrip() {
        // W869 face2: ------------------×logic·call.
        let src = "// ahead\nfunc main() {\n  while ------------------a && b {\n    print(1)\n  }\n  while ------------------ready() || done\n  print(1)\n  while (------------------n) && flag\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg18 logic");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------------------a && b"));
        assert!(out.contains("while ------------------ready() || done"));
        assert!(out.contains("while (------------------n) && flag"));
    }

    #[test]
    fn subset_while_neg18_mixed_roundtrip() {
        // W869 face3: ------------------ × effect × where × depth2.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while ------------------ready() && a || b {\n    print(foo(bar(baz())))\n  }\n}\nfunc main() {\n  while ------------------n\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse neg18 mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ------------------ready() && a || b"));
        assert!(out.contains("while ------------------n"));
    }


    #[test]
    fn subset_while_braceless_bare_roundtrip() {
        // W843 / ADR-1187 face1: brace-less bare cond.
        let src = "// ahead\nfunc main() {\n  while true\n  print(1)\n  while flag\n  print(x)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse braceless bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while true"));
        assert!(out.contains("while flag"));
    }

    #[test]
    fn subset_while_braceless_cond_roundtrip() {
        // W843 face2: call/compare cond brace-less.
        let src = "// ahead\nfunc main() {\n  while ready()\n  print(1)\n  while x < n\n  print(x)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse braceless cond");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while ready()"));
        assert!(out.contains("while x < n"));
    }

    #[test]
    fn subset_while_braceless_mixed_roundtrip() {
        // W843 face3: brace-less × braced × effect × where.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  while true\n  print(foo(bar(baz())))\n}\nfunc main() {\n  while ready() {\n    print(1)\n  }\n  while flag\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse braceless mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("while true"));
        assert!(out.contains("while ready() {"));
    }

    #[test]
    fn subset_depth2_call_print_roundtrip() {
        // W842 / ADR-1186 face1: print(foo(bar(baz()))) depth≤2.
        let src = "// ahead\nfunc main() {\n  print(foo(bar(baz())))\n  print(g(h(1)))\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse depth2 call print");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(foo(bar(baz())))"));
        assert!(out.contains("print(g(h(1)))"));
    }

    #[test]
    fn subset_depth2_call_rhs_roundtrip() {
        // W842 face2: let/var/return depth=2 nested call RHS.
        let src = "// ahead\nfunc main() {\n  let x = foo(bar(baz()))\n  var y = g(h(1))\n  return f(k(m()))\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse depth2 call rhs");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let x = foo(bar(baz()))"));
        assert!(out.contains("return f(k(m()))"));
    }

    #[test]
    fn subset_depth2_call_mixed_roundtrip() {
        // W842 face3: depth=2 × while × effect × where.
        let src = "// ahead\nfunc go<T>() requires Pure where T: Bound {\n  print(foo(bar(baz())))\n}\nfunc main() {\n  while ready() {\n    let x = f(g(h()))\n    print(x)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse depth2 call mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("print(foo(bar(baz())))"));
        assert!(out.contains("let x = f(g(h()))"));
    }

    #[test]
    fn subset_rejects_nested_call_depth_gt2() {
        let file = SourceFile::new("t.sy", "func main() { print(foo(bar(baz(qux())))) }\n");
        let err = parse_cst_subset(&file).expect_err("depth>2 nested call must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_effect_pure_roundtrip() {
        // W837 / ADR-1181: requires Pure + body.
        let src = "// ahead\nfunc main() requires Pure {\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse effect pure");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("requires Pure"));
    }

    #[test]
    fn subset_effect_io_roundtrip() {
        // W837: requires IO + params/return.
        let src = "// ahead\nfunc go(x: Int) -> Int requires IO {\n  print(x)\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse effect io");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("requires IO"));
    }

    #[test]
    fn subset_effect_mixed_roundtrip() {
        // W837: Pure × IO + while body.
        let src = "// ahead\nfunc pure() requires Pure {\n  print(0)\n}\nfunc io() requires IO {\n  while true {\n    print(1)\n  }\n}\nprint(2)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse effect mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("requires Pure"));
        assert!(out.contains("requires IO"));
    }

    #[test]
    fn subset_throws_bare_roundtrip() {
        // W839 / ADR-1183 face1: bare throws + body.
        let src = "// ahead\nfunc boom() throws {\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse throws bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("throws"));
    }

    #[test]
    fn subset_throws_clause_roundtrip() {
        // W839 face2: throws + return / requires.
        let src = "// ahead\nfunc fail() -> Int throws {\n  return 1\n}\nfunc g() throws requires Pure {\n  print(0)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse throws clause");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("throws"));
        assert!(out.contains("requires Pure"));
    }

    #[test]
    fn subset_throws_mixed_roundtrip() {
        // W839 face3: throws × nested-call × while × effect.
        let src = "// ahead\nfunc boom() throws {\n  print(foo(bar()))\n}\nfunc go() throws requires IO {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\nprint(1)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse throws mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("throws"));
        assert!(out.contains("print(foo(bar()))"));
    }

    #[test]
    fn subset_type_params_single_roundtrip() {
        // W840 / ADR-1184 face1: single type param.
        let src = "// ahead\nfunc id<T>(x: T) {\n  print(x)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse type_params single");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("id<T>"));
    }

    #[test]
    fn subset_type_params_multi_roundtrip() {
        // W840 face2: multi TP + return / throws.
        let src = "// ahead\nfunc pair<A, B>(a: A, b: B) {\n  print(a)\n}\nfunc id<T>(x: T) -> T throws {\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse type_params multi");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("pair<A, B>"));
        assert!(out.contains("throws"));
    }

    #[test]
    fn subset_type_params_mixed_roundtrip() {
        // W840 face3: type_params × throws × effect × nested-call.
        let src = "// ahead\nfunc id<T>(x: T) throws requires Pure {\n  print(foo(bar()))\n}\nfunc go<U>() {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\nprint(1)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse type_params mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("id<T>"));
        assert!(out.contains("print(foo(bar()))"));
    }

    #[test]
    fn subset_rejects_type_params_nested_tp() {
        // Nested generic TP remains out of subset.
        let file = SourceFile::new("t.sy", "func id<Foo<T>>(x: Foo<T>) { print(1) }\n");
        let err = parse_cst_subset(&file).expect_err("nested TP must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_where_bare_roundtrip() {
        // W841 / ADR-1185 face1: bare where.
        let src = "// ahead\nfunc id<T>(x: T) where T: Equatable {\n  print(x)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse where bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("where T: Equatable"));
    }

    #[test]
    fn subset_where_clause_roundtrip() {
        // W841 face2: multi preds / return / requires.
        let src = "// ahead\nfunc pair<A, B>(a: A, b: B) where A: Equatable, B: Hashable {\n  print(a)\n}\nfunc id<T>(x: T) -> T requires Pure where T: Equatable {\n  return x\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse where clause");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("where A: Equatable, B: Hashable"));
        assert!(out.contains("requires Pure where T: Equatable"));
    }

    #[test]
    fn subset_where_mixed_roundtrip() {
        // W841 face3: where × throws × effect × nested-call.
        let src = "// ahead\nfunc id<T>(x: T) throws requires Pure where T: Equatable {\n  print(foo(bar()))\n}\nfunc go<U>() where U: Bound {\n  while ready() {\n    let x = f(g())\n    print(x)\n  }\n}\nprint(1)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse where mixed");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("where T: Equatable"));
        assert!(out.contains("print(foo(bar()))"));
    }

    #[test]
    fn subset_typed_function_return_nested_roundtrip() {
        // W810 / ADR-1154: Function return may be nested Function TypeExpr.
        let src = "// ahead\nfunc main() {\n  // nested function return\n  let f: (Int) -> ((Int) -> Int) = id\n  var g: (String?) -> (() -> Void) = h\n  let u: () -> ((Int?) -> Int?) = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function return nested");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: (Int) -> ((Int) -> Int) = id"));
        assert!(out.contains("var g: (String?) -> (() -> Void) = h"));
        assert!(out.contains("let u: () -> ((Int?) -> Int?) = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_return_nested_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: (Int) -> ((Int) -> Int) = id\n}\nfunc main() {\n  var x: () -> (() -> Bool?) = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function return nested multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_function_return_reference_roundtrip() {
        // W807 / ADR-1151: Function return may be Amp Reference family.
        let src = "// ahead\nfunc main() {\n  // func return ref\n  let f: (Int) -> &Int = id\n  var g: (String?) -> &mut String = h\n  let u: () -> &'a Int = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function return reference");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: (Int) -> &Int = id"));
        assert!(out.contains("var g: (String?) -> &mut String = h"));
        assert!(out.contains("let u: () -> &'a Int = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_return_reference_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: (Int) -> &mut Int = id\n}\nfunc main() {\n  var x: () -> &'b String = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function return reference multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_typed_function_return_optional_roundtrip() {
        // W808 / ADR-1152: Function return may be Optional Named (`Ident?`).
        let src = "// ahead\nfunc main() {\n  // func return optional\n  let f: (Int) -> Int? = id\n  var g: (String?) -> String? = h\n  let u: () -> Void? = k\n  print(f)\n  return\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function return optional");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("let f: (Int) -> Int? = id"));
        assert!(out.contains("var g: (String?) -> String? = h"));
        assert!(out.contains("let u: () -> Void? = k"));
        assert!(out.contains("print(f)"));
    }

    #[test]
    fn subset_typed_function_return_optional_multi_func_roundtrip() {
        let src = "func helper() {\n  let n: (Int) -> Int? = id\n}\nfunc main() {\n  var x: () -> Bool? = z\n  return\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse typed function return optional multi func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_func_access_public_roundtrip() {
        // W875 / ADR-1219 face1: public func + docs comments byte-identical.
        let src = "/// docs\npublic func api() {\n  // body\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse public func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("public func api"));
        assert!(out.contains("/// docs"));
    }

    #[test]
    fn subset_func_access_private_family_roundtrip() {
        // W875 face2: private / internal / fileprivate.
        let src = "// ahead\nprivate func a() {\n  print(1)\n}\ninternal func b() {\n  /* mid */\n  print(2)\n}\nfileprivate func c() {\n  print(3)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse private family");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_func_access_mixed_roundtrip() {
        // W875 face3: access × throws/effect/where/while + comments.
        let src = "// ahead\npublic func go<T>() throws requires Pure where T: Bound {\n  // body\n  while flag /* mid */\n  print(1)\n}\nprivate func main() {\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse access mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_func_static_roundtrip() {
        // W876 / ADR-1220 face1: static func + docs comments byte-identical.
        let src = "/// docs\nstatic func api() {\n  // body\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse static func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("static func api"));
        assert!(out.contains("/// docs"));
    }

    #[test]
    fn subset_func_async_roundtrip() {
        // W876 face2: async func + comments.
        let src = "// ahead\nasync func a() {\n  print(1)\n}\nasync func b() {\n  /* mid */\n  print(2)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse async func");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_func_static_async_mixed_roundtrip() {
        // W876 face3: access × static/async × throws/effect/where/while + comments.
        let src = "// ahead\npublic static func go<T>() throws requires Pure where T: Bound {\n  // body\n  while flag /* mid */\n  print(1)\n}\nprivate async func main() {\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse static/async mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_func_export_bare_roundtrip() {
        // W878 / ADR-1222 face1: export func + docs comments byte-identical.
        let src = "/// docs\nexport func api() {\n  // body\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export func");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("export func api") && out.contains("/// docs"));
    }

    #[test]
    fn subset_func_export_mod_roundtrip() {
        // W878 face2: export static / export async + comments.
        let src = "// ahead\nexport static func a() {\n  print(1)\n}\nexport async func b() {\n  /* mid */\n  print(2)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export mod");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_func_export_mixed_roundtrip() {
        // W878 face3: export × throws/effect/where/while/if + comments.
        let src = "// ahead\nexport func go<T>() throws requires Pure where T: Bound {\n  // body\n  if flag {\n    while ready /* mid */\n    print(1)\n  }\n}\nexport func main() {\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_export_list_bare_roundtrip() {
        // W883 / ADR-1227 face1: bare export list + docs.
        let src = "/// docs\nexport { foo }\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export list bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("export { foo }") && out.contains("/// docs"));
    }

    #[test]
    fn subset_export_list_multi_roundtrip() {
        // W883 face2: multi names + trailing comma + comments.
        let src = "// ahead\nexport { a, b, c, }\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export list multi");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_export_list_mixed_roundtrip() {
        // W883 face3: export list × export func × data + comments.
        let src = "// ahead\nexport { Point }\ndata class Point(x: Int)\nexport func main() {\n  /* mid */\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export list mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_export_as_bare_roundtrip() {
        // W885 / ADR-1229 face1: bare as-rename + docs.
        let src = "/// docs\nexport { foo as bar }\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export as bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("export { foo as bar }") && out.contains("/// docs"));
    }

    #[test]
    fn subset_export_empty_roundtrip() {
        // W885 face2: empty export list + comments.
        let src = "// ahead\nexport { }\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse empty export");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_export_as_empty_mixed_roundtrip() {
        // W885 face3: as × empty × thin list × data/func + comments.
        let src = "// ahead\nexport { }\nexport { foo as bar }\nexport { Point }\ndata class Point(x: Int)\nexport func main() {\n  /* mid */\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse export as empty mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_rejects_export_list() {
        // W898: nested self.a.b= OK; member-index compound remains CST001.
        let ok = SourceFile::new(
            "s.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.inner.name = name\n  }\n}\n",
        );
        assert!(parse_cst_subset(&ok).is_ok(), "nested self.a.b= must parse");
        let compound = SourceFile::new(
            "c.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
        );
        let err = parse_cst_subset(&compound).expect_err("member-index compound must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_class_bare_roundtrip() {
        // W879 / ADR-1223 face1: empty / stored-only class + docs.
        let src = "/// docs\nclass Box {\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse bare class");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("class Box") && out.contains("/// docs"));
        let stored = "// ahead\nclass Node {\n  var name: String\n}\n";
        let file2 = SourceFile::new("s.sy", stored);
        let root2 = parse_cst_subset(&file2).expect("parse stored-only class");
        assert_eq!(format_cst(&root2), stored);
    }

    #[test]
    fn subset_class_members_roundtrip() {
        // W879 face2: class + var + simple method + comments.
        let src = "// ahead\nclass Box {\n  var name: String\n  // method\n  func tag() {\n    /* mid */\n    print(1)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class members");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_class_mixed_roundtrip() {
        // W879 face3: public class × method × top-level func + comments.
        let src = "// ahead\npublic class Box {\n  var name: String?\n  func tag() {\n    // body\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_class_inherit_roundtrip() {
        // W880 / ADR-1224 face1: single superclass + docs.
        let src = "/// docs\nclass Sub: Box {\n  var name: String\n  func tag() {\n    print(1)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class inherit");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("class Sub: Box") && out.contains("/// docs"));
    }

    #[test]
    fn subset_class_init_roundtrip() {
        // W880 face2: init with typed params + simple body + comments.
        let src = "// ahead\nclass Box {\n  var name: String\n  init(name: String) {\n    /* mid */\n    print(1)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class init");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_class_deep_mixed_roundtrip() {
        // W880 face3: public inherit × init × top-level func + comments.
        let src = "// ahead\npublic class Sub: Box {\n  var name: String?\n  init(name: String) {\n    // body\n    print(1)\n  }\n  func tag() {\n    print(2)\n  }\n}\nfunc main() {\n  /* mid */\n  print(3)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class deep mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_rejects_class_inheritance() {
        // W898: nested self.a.b= OK; member-index compound remains CST001.
        let ok = SourceFile::new(
            "t.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.inner.name = name\n  }\n}\n",
        );
        assert!(parse_cst_subset(&ok).is_ok(), "nested self.a.b= must parse");
        let compound = SourceFile::new(
            "c.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
        );
        let err = parse_cst_subset(&compound).expect_err("member-index compound must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_class_init() {
        // W898: non-self member assign OK; member-index compound remains CST001.
        let ok = SourceFile::new(
            "t.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    box.name = name\n  }\n}\n",
        );
        assert!(parse_cst_subset(&ok).is_ok(), "non-self member assign must parse");
        let compound = SourceFile::new(
            "c.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
        );
        let err = parse_cst_subset(&compound).expect_err("member-index compound must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_class_multi_conf_roundtrip() {
        // W886 / ADR-1230 face1: multi-conformance + docs.
        let src = "/// docs\nclass Sub: Box, Greeter {\n  var name: String\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class multi conf");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("class Sub: Box, Greeter") && out.contains("/// docs"));
    }

    #[test]
    fn subset_class_deinit_roundtrip() {
        // W886 face2: deinit + comments.
        let src = "// ahead\nclass Box {\n  var name: String\n  deinit {\n    /* mid */\n    print(1)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class deinit");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_class_multi_deinit_mixed_roundtrip() {
        // W886 face3: multi × deinit × init × top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    print(1)\n  }\n  deinit {\n    print(2)\n  }\n}\nfunc main() {\n  /* mid */\n  print(3)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse class multi deinit mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_self_field_bare_roundtrip() {
        // W887 / ADR-1231 face1: init self.field= + docs.
        let src = "/// docs\nclass Box {\n  var name: String\n  init(name: String) {\n    self.name = name\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse self field bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("self.name = name") && out.contains("/// docs"));
    }

    #[test]
    fn subset_self_field_method_roundtrip() {
        // W887 face2: method self.field= + comments.
        let src = "// ahead\nclass Box {\n  var name: String\n  func setName(name: String) {\n    /* mid */\n    self.name = name\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse self field method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_self_field_mixed_roundtrip() {
        // W887 face3: init×method×multi/deinit + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    self.name = name\n  }\n  func retag(tag: String) {\n    self.name = tag\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse self field mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_nested_self_field_bare_roundtrip() {
        // W888 / ADR-1232 face1: init nested self.a.b= + docs.
        let src = "/// docs\nclass Box {\n  var name: String\n  init(name: String) {\n    self.inner.name = name\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested self field bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("self.inner.name = name") && out.contains("/// docs"));
    }

    #[test]
    fn subset_nested_self_field_method_roundtrip() {
        // W888 face2: method nested self.a.b= + comments.
        let src = "// ahead\nclass Box {\n  var name: String\n  func setName(name: String) {\n    /* mid */\n    self.inner.name = name\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested self field method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_nested_self_field_mixed_roundtrip() {
        // W888 face3: nested×单层×multi/deinit + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    self.inner.name = name\n  }\n  func retag(tag: String) {\n    self.name = tag\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested self field mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_bare_roundtrip() {
        // W889 / ADR-1233 face1: init non-self member assign + docs.
        let src = "/// docs\nclass Box {\n  var name: String\n  init(name: String) {\n    box.name = name\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("box.name = name") && out.contains("/// docs"));
    }

    #[test]
    fn subset_member_field_method_roundtrip() {
        // W889 face2: method non-self member assign + comments.
        let src = "// ahead\nclass Box {\n  var name: String\n  func setName(name: String) {\n    /* mid */\n    other.name = name\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_mixed_roundtrip() {
        // W889 face3: non-self×self×nested×multi/deinit + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var name: String?\n  init(name: String) {\n    // body\n    box.name = name\n  }\n  func retag(tag: String) {\n    self.inner.name = tag\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_plus_bare_roundtrip() {
        // W890 / ADR-1234 face1: init self.count += + docs.
        let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    self.count += 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field plus bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("self.count += 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_member_field_plus_method_roundtrip() {
        // W890 face2: method box.x += + comments.
        let src = "// ahead\nclass Box {\n  var count: Int\n  func bump(n: Int) {\n    /* mid */\n    box.count += n\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field plus method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_plus_mixed_roundtrip() {
        // W890 face3: self×non-self×nested×multi/deinit + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    self.count += 1\n  }\n  func bump(n: Int) {\n    box.inner.count += n\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field plus mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_minus_bare_roundtrip() {
        // W891 / ADR-1235 face1: init self.count -= + docs.
        let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    self.count -= 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field minus bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("self.count -= 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_member_field_minus_method_roundtrip() {
        // W891 face2: method box.x -= + comments.
        let src = "// ahead\nclass Box {\n  var count: Int\n  func drop(n: Int) {\n    /* mid */\n    box.count -= n\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field minus method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_minus_mixed_roundtrip() {
        // W891 face3: self×non-self×nested×multi/deinit + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    self.count -= 1\n  }\n  func drop(n: Int) {\n    box.inner.count -= n\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field minus mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_star_bare_roundtrip() {
        // W892 / ADR-1236 face1: init self.count *= + docs.
        let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    self.count *= 2\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field star bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("self.count *= 2") && out.contains("/// docs"));
    }

    #[test]
    fn subset_member_field_star_method_roundtrip() {
        // W892 face2: method box.x *= / /= + comments.
        let src = "// ahead\nclass Box {\n  var count: Int\n  func scale(n: Int) {\n    /* mid */\n    box.count *= n\n    box.count /= 2\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field star method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_field_star_mixed_roundtrip() {
        // W892 face3: self×non-self×nested×%=×multi/deinit + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    self.count *= 2\n  }\n  func scale(n: Int) {\n    box.inner.count %= n\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member field star mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_ident_compound_bare_roundtrip() {
        // W893 / ADR-1237 face1: init count += + docs.
        let src = "/// docs\nclass Box {\n  var count: Int\n  init() {\n    count += 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse ident compound bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("count += 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_ident_compound_method_roundtrip() {
        // W893 face2: method count *= / /= + comments.
        let src = "// ahead\nclass Box {\n  var count: Int\n  func scale(n: Int) {\n    /* mid */\n    count *= n\n    count /= 2\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse ident compound method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_ident_compound_mixed_roundtrip() {
        // W893 face3: ident×member×nested×multi/deinit×top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var count: Int?\n  init() {\n    // body\n    count += 1\n  }\n  func scale(n: Int) {\n    box.inner.count %= n\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse ident compound mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_assign_bare_roundtrip() {
        // W894 / ADR-1238 face1: init a[0] = + docs.
        let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0] = 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index assign bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("a[0] = 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_index_assign_method_roundtrip() {
        // W894 face2: method xs[i] = + comments.
        let src = "// ahead\nclass Box {\n  var items: Int\n  func set(i: Int, n: Int) {\n    /* mid */\n    xs[i] = n\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index assign method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_assign_mixed_roundtrip() {
        // W894 face3: index×ident-compound×member×multi/deinit×top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0] = 1\n  }\n  func set(i: Int, n: Int) {\n    xs[i] = n\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index assign mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_compound_bare_roundtrip() {
        // W895 / ADR-1239 face1: init a[0] += + docs.
        let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0] += 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index compound bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("a[0] += 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_index_compound_method_roundtrip() {
        // W895 face2: method xs[i] *= / /= + comments.
        let src = "// ahead\nclass Box {\n  var items: Int\n  func scale(i: Int, n: Int) {\n    /* mid */\n    xs[i] *= n\n    xs[i] /= 2\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index compound method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_compound_mixed_roundtrip() {
        // W895 face3: index compound×ident-compound×multi/deinit×top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0] += 1\n  }\n  func scale(i: Int, n: Int) {\n    xs[i] %= n\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index compound mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_nested_bare_roundtrip() {
        // W896 / ADR-1240 face1: init a[0][1] = + docs.
        let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0][1] = 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index nested bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("a[0][1] = 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_index_nested_method_roundtrip() {
        // W896 face2: method buf[i][j] = + comments.
        let src = "// ahead\nclass Box {\n  var items: Int\n  func set(i: Int, j: Int, n: Int) {\n    /* mid */\n    buf[i][j] = n\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index nested method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_nested_mixed_roundtrip() {
        // W896 face3: nested=×index-compound×multi/deinit×top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0][1] = 1\n  }\n  func scale(i: Int, j: Int, n: Int) {\n    buf[i][j] = n\n    xs[i] += 1\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse index nested mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_index_assign_bare_roundtrip() {
        // W897 / ADR-1241 face1: init self.xs[0] = + docs.
        let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    self.xs[0] = 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member-index bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("self.xs[0] = 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_member_index_assign_method_roundtrip() {
        // W897 face2: method box.items[i] = + comments.
        let src = "// ahead\nclass Box {\n  var items: Int\n  func set(i: Int, n: Int) {\n    /* mid */\n    box.items[i] = n\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member-index method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_member_index_assign_mixed_roundtrip() {
        // W897 face3: member-index=×nested×index-compound×multi/deinit×top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    self.xs[0] = 1\n  }\n  func scale(i: Int, j: Int, n: Int) {\n    box.items[i] = n\n    a[0][1] = 2\n    xs[i] += 1\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse member-index mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_nested_compound_bare_roundtrip() {
        // W898 / ADR-1242 face1: init a[0][1] += + docs.
        let src = "/// docs\nclass Box {\n  var items: Int\n  init() {\n    a[0][1] += 1\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested compound bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("a[0][1] += 1") && out.contains("/// docs"));
    }

    #[test]
    fn subset_index_nested_compound_method_roundtrip() {
        // W898 face2: method buf[i][j] *= / /= + comments.
        let src = "// ahead\nclass Box {\n  var items: Int\n  func scale(i: Int, j: Int, n: Int) {\n    /* mid */\n    buf[i][j] *= n\n    buf[i][j] /= 2\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested compound method");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_index_nested_compound_mixed_roundtrip() {
        // W898 face3: nested-compound×member-index=×multi/deinit×top-level + comments.
        let src = "// ahead\npublic class Sub: Box, Greeter {\n  var items: Int?\n  init() {\n    // body\n    a[0][1] += 1\n  }\n  func scale(i: Int, j: Int, n: Int) {\n    buf[i][j] %= n\n    self.xs[0] = 2\n    count *= 2\n  }\n  deinit {\n    print(1)\n  }\n}\nfunc main() {\n  /* mid */\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse nested compound mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_data_class_roundtrip() {
        // W881 / ADR-1225 face1: thin data class + docs.
        let src = "/// docs\ndata class Point(x: Int, y: Int)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse data class");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("data class Point") && out.contains("/// docs"));
        let empty = "// ahead\ndata class Empty()\n";
        let file2 = SourceFile::new("e.sy", empty);
        let root2 = parse_cst_subset(&file2).expect("parse empty data class");
        assert_eq!(format_cst(&root2), empty);
    }

    #[test]
    fn subset_final_class_roundtrip() {
        // W881 face2: thin final class variants + comments.
        let src = "// ahead\nfinal class Token {\n  Ident\n  Number(value: Int)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse final class");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_data_final_mixed_roundtrip() {
        // W881 face3: public data × final × top-level func + comments.
        let src = "// ahead\npublic data class Point(x: Int)\nfinal class Shape {\n  point\n  circle(r: Int)\n}\nfunc main() {\n  /* mid */\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse data/final mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_data_class_tp_roundtrip() {
        // W882 / ADR-1226 face1: data class type params + docs.
        let src = "/// docs\ndata class Box<T>(value: T)\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse data class tp");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("data class Box<T>") && out.contains("/// docs"));
        let multi = "// ahead\ndata class Pair<A, B>(a: A, b: B)\n";
        let file2 = SourceFile::new("p.sy", multi);
        assert_eq!(format_cst(&parse_cst_subset(&file2).expect("pair tp")), multi);
    }

    #[test]
    fn subset_data_class_where_roundtrip() {
        // W882 face2: data class where + comments.
        let src = "// ahead\ndata class Box<T>(value: T) where T: Equatable\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse data class where");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_data_final_gw_mixed_roundtrip() {
        // W882 face3: data where × final tp × top-level func + comments.
        let src = "// ahead\npublic data class Box<T>(value: T) where T: Equatable\nfinal class Wrap<U> {\n  wrap(value: U)\n}\nfunc main() {\n  /* mid */\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse data/final gw mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_final_where_bare_roundtrip() {
        // W884 / ADR-1228 face1: final class where + docs.
        let src = "/// docs\nfinal class Box<T> where T: Equatable {\n  wrap(value: T)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse final where bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("where T: Equatable") && out.contains("/// docs"));
    }

    #[test]
    fn subset_final_where_multi_roundtrip() {
        // W884 face2: multi-pred where + public + comments.
        let src = "// ahead\npublic final class Pair<A, B> where A: Equatable, B: Hashable {\n  pair(a: A, b: B)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse final where multi");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_final_where_mixed_roundtrip() {
        // W884 face3: final+where × data where × export list + comments.
        let src = "// ahead\nexport { Box }\npublic data class Box<T>(value: T) where T: Equatable\nfinal class Wrap<U> where U: Hashable {\n  wrap(value: U)\n}\nfunc main() {\n  /* mid */\n  print(1)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse final where mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_rejects_data_class() {
        // W898: nested self.a.b= OK; member-index compound remains CST001.
        let ok = SourceFile::new(
            "s.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.inner.name = name\n  }\n}\n",
        );
        assert!(parse_cst_subset(&ok).is_ok(), "nested self.a.b= must parse");
        let compound = SourceFile::new(
            "c.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
        );
        let err = parse_cst_subset(&compound).expect_err("member-index compound must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_final_class_generics() {
        // W898: non-self member assign OK; member-index compound remains CST001.
        let ok = SourceFile::new(
            "s.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    box.name = name\n  }\n}\n",
        );
        assert!(parse_cst_subset(&ok).is_ok(), "non-self member assign must parse");
        let compound = SourceFile::new(
            "c.sy",
            "class Box {\n  var name: String\n  init(name: String) {\n    self.xs[0] += 1\n  }\n}\n",
        );
        let err = parse_cst_subset(&compound).expect_err("member-index compound must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_static_async_stack() {
        // W876: static+async stack still CST001 (0..1 of Static|Async).
        let file = SourceFile::new("t.sy", "static async func main() { print(1) }\n");
        let err = parse_cst_subset(&file).expect_err("static async stack must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_if_bare_roundtrip() {
        // W877 / ADR-1221 face1: if bare + docs comments byte-identical.
        let src = "/// docs\nfunc main() {\n  // body\n  if flag {\n    print(1)\n  }\n  if ready\n  print(2)\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse if bare");
        let out = format_cst(&root);
        assert_eq!(out, src);
        assert!(out.contains("if flag") && out.contains("/// docs"));
    }

    #[test]
    fn subset_if_else_roundtrip() {
        // W877 face2: if/else + comments.
        let src = "// ahead\nfunc main() {\n  if a {\n    print(1)\n  } else {\n    /* mid */\n    print(2)\n  }\n  if b\n  print(3)\n  else\n  print(4)\n}\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse if else");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_if_mixed_roundtrip() {
        // W877 face3: if × logic/while/else-if/access + comments.
        let src = "// ahead\npublic func go() {\n  // body\n  if a && b {\n    while flag /* mid */\n    print(1)\n  } else if c {\n    print(2)\n  } else {\n    print(3)\n  }\n}\n/* trail */\n";
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse if mixed");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_rejects_if_case() {
        // W877: if case remains out of subset (M0 still AST F001).
        let file = SourceFile::new(
            "t.sy",
            "func main() {\n  if case \\Foo.bar = x {\n    print(1)\n  }\n}\n",
        );
        let err = parse_cst_subset(&file).expect_err("if case must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_rejects_if_let() {
        let file = SourceFile::new(
            "t.sy",
            "func main() {\n  if let x = y {\n    print(1)\n  }\n}\n",
        );
        let err = parse_cst_subset(&file).expect_err("if let must CST001");
        assert_eq!(err.code.as_deref(), Some("CST001"));
    }

    #[test]
    fn subset_builtin_attr_class_roundtrip() {
        // W900: `@builtin("…")` + class + static + extension + labeled param + self arg.
        let src = r#"/// docs
@builtin("numeric.int")
public class Int: Equatable {
public static func min(a: Int, b: Int) -> Int {
return __syx_int_min(a, b)
}
}
extension Int {
func hash(into hasher: Hasher) {
__syx_hasher_combine(hasher, self)
}
}
"#;
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse @builtin");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_builtin_dict_type_roundtrip() {
        // W900: Dictionary facade param (Object-key honesty; still parses Named type).
        let src = r#"@builtin("collection.dictionary")
public class Dictionary {
public static func count(map: Dictionary) -> Int {
return __syx_dict_count(map)
}
}
"#;
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse dict type");
        assert_eq!(format_cst(&root), src);
    }

    #[test]
    fn subset_try_catch_throw_roundtrip() {
        // W900: try/catch/throw for file_handle comment sample.
        let src = r#"/// ahead
public class FileHandle {
    public static func open(path: String) -> Int throws {
        try {
            return __syx_file_open(path)
        } catch (e) {
            throw IOError(code: 1, message: "fail")
        }
    }
}
"#;
        let file = SourceFile::new("t.sy", src);
        let root = parse_cst_subset(&file).expect("parse try/catch");
        assert_eq!(format_cst(&root), src);
    }
}
