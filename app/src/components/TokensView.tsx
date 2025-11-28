import React from 'react';
import { Badge } from "./ui/badge";
import { ScrollArea } from "./ui/scroll-area";
import { AlertCircle } from "lucide-react";

export interface Token {
    type: string;
    value: string;
    position: {
        start: number;
        end: number;
        line: number;
        column: number;
    };
    malformed?: string;
}

interface TokensViewProps {
    tokens: Token[] | undefined;
}

const translateTokenType = (type: string): string => {
    const translations: Record<string, string> = {
        keyword: "Palavra-chave",
        identifier: "Identificador",
        literal: "Literal",
        operator: "Operador",
        punctuation: "Pontuação",
        comment: "Comentário",
        whitespace: "Espaço",
        newline: "Quebra de linha",
    };
    return translations[type] || type.toUpperCase();
};

const tokenColorMap: Record<string, string> = {
    keyword: "bg-blue-500/15 text-blue-300 border-blue-500/30",
    identifier: "bg-purple-500/15 text-purple-300 border-purple-500/30",
    literal: "bg-green-500/15 text-green-300 border-green-500/30",
    operator: "bg-orange-500/15 text-orange-300 border-orange-500/30",
    punctuation: "bg-pink-500/15 text-pink-300 border-pink-500/30",
    comment: "bg-slate-500/15 text-slate-300 border-slate-500/30",
};

export const TokensView: React.FC<TokensViewProps> = ({ tokens }) => {
    if (!tokens) {
        return (
            <div className="h-full flex items-center justify-center text-gray-400 p-8">
                <div className="text-center space-y-3">
                    <AlertCircle className="w-10 h-10 mx-auto opacity-40" />
                    <p className="text-sm font-medium text-gray-300">Nenhum token para exibir</p>
                    <p className="text-xs text-gray-500">Clique em "Analisar" para ver os tokens</p>
                </div>
            </div>
        );
    }

    const filteredTokens = tokens.filter(
        token => token.type !== 'whitespace' && token.type !== 'newline'
    );

    if (filteredTokens.length === 0) {
        return (
            <div className="h-full flex items-center justify-center text-gray-400 p-8">
                <div className="text-center space-y-3">
                    <AlertCircle className="w-10 h-10 mx-auto opacity-40" />
                    <p className="text-sm font-medium text-gray-300">Nenhum token encontrado</p>
                </div>
            </div>
        );
    }

    return (
        <ScrollArea className="h-full">
            <div className="p-4">
                <div className="space-y-1.5">
                    {filteredTokens.map((token, index) => (
                        <div
                            key={index}
                            className={`flex items-center gap-3 p-2.5 rounded border transition-colors ${token.malformed
                                ? 'bg-red-950/20 border-red-900/30 hover:bg-red-950/30'
                                : 'bg-neutral-950 border-neutral-800 hover:bg-neutral-900'
                                }`}
                        >
                            <div className="flex-shrink-0 w-6 h-6 rounded bg-neutral-900 flex items-center justify-center text-xs text-gray-400 font-medium border border-neutral-800">
                                {index + 1}
                            </div>
                            <div className="flex-1 min-w-0">
                                <div className="flex items-center gap-2 mb-1 flex-wrap">
                                    <Badge
                                        variant="outline"
                                        className={tokenColorMap[token.type] || "bg-neutral-900 text-gray-400 border-neutral-800"}
                                    >
                                        {translateTokenType(token.type)}
                                    </Badge>
                                    <code className="text-xs bg-black px-2 py-0.5 rounded border border-neutral-800 text-gray-200 font-mono">
                                        {token.value}
                                    </code>
                                </div>
                                <div className="text-xs text-gray-500">
                                    Linha {token.position.line}, Coluna {token.position.column}
                                </div>
                                {token.malformed && (
                                    <div className="mt-1.5 flex items-start gap-1.5 text-red-400 text-xs bg-red-950/10 px-2 py-1 rounded border border-red-900/20">
                                        <span className="font-semibold">⚠</span>
                                        <span>{token.malformed}</span>
                                    </div>
                                )}
                            </div>
                        </div>
                    ))}
                </div>

                <div className="mt-4 p-3 bg-neutral-950 rounded border border-neutral-800">
                    <div className="text-xs text-gray-400">
                        <strong className="text-gray-300">Total:</strong> {filteredTokens.length} tokens
                    </div>
                </div>
            </div>
        </ScrollArea>
    );
};
