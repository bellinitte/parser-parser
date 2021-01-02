(function (mod) {
    if (typeof exports == "object" && typeof module == "object")
        // CommonJS
        mod(require("codemirror"));
    else if (typeof define == "function" && define.amd)
        // AMD
        define(["codemirror"], mod);
    // Plain browser env
    else mod(CodeMirror);
})(function (CodeMirror) {
    "use strict";

    const phraseType = { comment: 0, string: 1, special: 2, nonterminal: 3 };

    CodeMirror.defineMode("ebnf", function (_) {
        return {
            startState: function () {
                return {
                    phrase: null,
                    commentLevel: null,
                    lastStringQuote: null,
                    lastStringEmpty: null,
                };
            },
            token: function (stream, state) {
                if (!stream) return;

                while (true) {
                    switch (state.phrase) {
                        case phraseType.comment:
                            while (state.commentLevel > 0) {
                                if (!stream.match(/\(\*\)/)) {
                                    if (stream.match(/\(\*/)) {
                                        state.commentLevel += 1;
                                    } else if (stream.match(/\*\)/)) {
                                        state.commentLevel -= 1;
                                    } else {
                                        if (stream.eol()) {
                                            return "comment";
                                        }
                                        stream.next();
                                    }
                                }
                            }
                            state.phrase = null;
                            state.commentLevel = null;
                            return "comment";

                        case phraseType.string:
                            while (
                                !stream.eol() &&
                                stream.peek() !== state.lastStringQuote
                            ) {
                                stream.next();
                                state.lastStringEmpty = false;
                            }
                            if (stream.peek() === state.lastStringQuote) {
                                stream.next();
                                state.phrase = null;
                                state.lastStringQuote = null;
                                state.lastStringEmpty = null;
                            }
                            return state.lastStringEmpty ? null : "string";

                        case phraseType.special:
                            while (!stream.eol() && stream.peek() !== "?") {
                                stream.next();
                            }
                            if (stream.peek() === "?") {
                                stream.next();
                                state.phrase = null;
                            }
                            return "string";

                        case phraseType.nonterminal:
                            stream.match(/[\p{L}\p{Nd}_\s]*/u);
                            if (!stream.eol()) {
                                state.phrase = null;
                            }
                            return "nonterminal";

                        case null:
                            if (!stream.match(/\(\*\)/)) {
                                if (stream.match(/\(\*/)) {
                                    state.phrase = phraseType.comment;
                                    state.commentLevel = 1;
                                    continue;
                                }
                            } else {
                                return null;
                            }

                            if (
                                stream.peek() === "'" ||
                                stream.peek() === '"'
                            ) {
                                state.phrase = phraseType.string;
                                state.lastStringQuote = stream.next();
                                state.lastStringEmpty = true;
                                continue;
                            }

                            if (stream.peek() === "?") {
                                stream.next();
                                state.phrase = phraseType.special;
                                continue;
                            }

                            if (stream.match(/\(:\)|\(\/\)/)) {
                                return null;
                            }

                            if (
                                stream.match(
                                    /\(|\[|\{|\)|\]|\}|\(:|:\)|\(\/|\/\)/
                                )
                            ) {
                                return "bracket";
                            }

                            if (stream.match(/[0-9][0-9\s]*/u)) {
                                return "number";
                            }

                            if (stream.match(/[\p{L}_]/u)) {
                                state.phrase = phraseType.nonterminal;
                                continue;
                            }

                            if (stream.match(/\s/u)) {
                                return null;
                            }

                            stream.next();
                            return "punctuation";
                    }
                }
            },
        };
    });

    CodeMirror.defineMIME("text/x-ebnf", "ebnf");
});
