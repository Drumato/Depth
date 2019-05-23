//use super::
pub struct SymbolDiscriminator {
    pub terminal: Vec<Token>,
    pub nonterminal: Vec<Token>,
}

impl SymbolDiscriminator {
    fn new(syndef: [SyntaxDefinition; 8]) -> SymbolDiscriminator {
        let mut terms: Vec<Token> = Vec::new();
        let mut nonterms: Vec<Token> = Vec::new();
        for rule in syndef.iter() {
            nonterms.push(rule.ltoken);
        }
    }
}
/*
export class SymbolDiscriminator {
    private terminal_symbols: Set<Token>;
    private nonterminal_symbols: Set<Token>;
    constructor(grammar: GrammarDefinition) {
        this.terminal_symbols = new Set<Token>();
        this.nonterminal_symbols = new Set<Token>();

        // 左辺値の登録
        for (const rule of grammar.rules) {
            const symbol = rule.ltoken;
            // 構文規則の左辺に現れる記号は非終端記号
            this.nonterminal_symbols.add(symbol);
        }
        // 右辺値の登録
        for (const rule of grammar.rules) {
            for (const symbol of rule.pattern) {
                if (!this.nonterminal_symbols.has(symbol)) {
                    // 非終端記号でない(=左辺値に現れない)場合、終端記号である
                    this.terminal_symbols.add(symbol);
                }
            }
        }
    }
    /**
     * 終端記号の集合をSetで得る
     * @param {boolean} prevent_copy trueを与えるとSetをコピーせず返す
     * 結果に変更が加えられないと保証される場合に用いる
     * @returns {Set<Token>}
     */
    public getTerminalSymbols(prevent_copy: boolean = false): Set<Token> {
        if (prevent_copy) return this.terminal_symbols;
        // コピーを返す
        return new Set(this.terminal_symbols);
    }
    /**
     * 非終端記号の集合をSetで得る
     * @param {boolean} prevent_copy trueを与えるとSetをコピーせず返す
     * 結果に変更が加えられないと保証される場合に用いる
     * @returns {Set<Token>}
     */
    public getNonterminalSymbols(prevent_copy: boolean = false): Set<Token> {
        if (prevent_copy) return this.nonterminal_symbols;
        // コピーを返す
        return new Set(this.nonterminal_symbols);
    }
    /**
     * 与えられた記号が終端記号かどうかを調べる
     * @param {Token} symbol
     * @returns {boolean}
     */
    public isTerminalSymbol(symbol: Token): boolean {
        return this.terminal_symbols.has(symbol);
    }
    /**
     * 与えられた記号が非終端記号かどうかを調べる
     * @param {Token} symbol
     * @returns {boolean}
     */
    public isNonterminalSymbol(symbol: Token): boolean {
        return this.nonterminal_symbols.has(symbol);
    }
}
*/
