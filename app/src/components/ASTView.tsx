import { ScrollArea } from "./ui/scroll-area";
import { AlertCircle, ChevronRight, ChevronDown } from "lucide-react";
import { useState } from "react";

interface ASTNodeProps {
    node: any;
    depth?: number;
    name?: string;
}

function ASTNodeComponent({ node, depth = 0, name }: ASTNodeProps) {
    const [isExpanded, setIsExpanded] = useState(depth < 3);

    if (!node) {
        return null;
    }

    if (typeof node !== 'object') {
        return (
            <div className="flex items-center gap-2">
                {name && <span className="text-xs text-gray-500">{name}:</span>}
                <code className="text-xs bg-emerald-500/10 px-1.5 py-0.5 rounded border border-emerald-500/20 text-emerald-300">
                    {String(node)}
                </code>
            </div>
        );
    }

    const getChildEntries = (obj: any): [string, any][] => {
        const entries: [string, any][] = [];

        for (const [key, value] of Object.entries(obj)) {
            if (key === 'type') continue;

            if (value && (typeof value === 'object' || Array.isArray(value))) {
                entries.push([key, value]);
            }
        }

        return entries;
    };

    const childEntries = getChildEntries(node);
    const hasChildren = childEntries.length > 0;

    const translateNodeType = (type: string): string => {
        const translations: Record<string, string> = {
            Program: "Programa",
            FunctionDeclaration: "Declaração de Função",
            VariableDeclaration: "Declaração de Variável",
            CallExpression: "Expressão de Chamada",
            BinaryExpression: "Expressão Binária",
            Identifier: "Identificador",
            Literal: "Literal",
            ReturnStatement: "Declaração de Retorno",
            ExpressionStatement: "Declaração de Expressão",
            BlockStatement: "Bloco de Declarações",
            VariableDeclarator: "Declarador de Variável",
            MemberExpression: "Expressão de Membro",
            Object: "Objeto",
        };
        return translations[type] || type;
    };

    const translatePropertyName = (name: string): string => {
        const translations: Record<string, string> = {
            body: "corpo",
            arguments: "argumentos",
            callee: "chamado",
            object: "objeto",
            property: "propriedade",
            computed: "computado",
            expression: "expressão",
            id: "id",
            params: "parâmetros",
            return_type: "tipo_retorno",
            declarations: "declarações",
            init: "inicialização",
            kind: "tipo",
            operator: "operador",
            left: "esquerda",
            right: "direita",
            argument: "argumento",
            name: "nome",
            value: "valor",
            raw: "bruto",
        };
        return translations[name] || name;
    };

    const nodeColors: Record<string, string> = {
        Program: "text-cyan-300 bg-cyan-500/10 border-cyan-500/20",
        FunctionDeclaration: "text-violet-300 bg-violet-500/10 border-violet-500/20",
        VariableDeclaration: "text-emerald-300 bg-emerald-500/10 border-emerald-500/20",
        CallExpression: "text-amber-300 bg-amber-500/10 border-amber-500/20",
        BinaryExpression: "text-rose-300 bg-rose-500/10 border-rose-500/20",
        Identifier: "text-sky-300 bg-sky-500/10 border-sky-500/20",
        Literal: "text-pink-300 bg-pink-500/10 border-pink-500/20",
        ReturnStatement: "text-orange-300 bg-orange-500/10 border-orange-500/20",
        ExpressionStatement: "text-indigo-300 bg-indigo-500/10 border-indigo-500/20",
        BlockStatement: "text-purple-300 bg-purple-500/10 border-purple-500/20",
        VariableDeclarator: "text-teal-300 bg-teal-500/10 border-teal-500/20",
    };

    const nodeType = node.type || "Object";
    const colorClass = nodeColors[nodeType] || "text-gray-300 bg-gray-500/10 border-gray-500/20";

    const getSimpleProps = () => {
        const props: Record<string, any> = {};

        for (const [key, value] of Object.entries(node)) {
            if (key === 'type') continue;
            if (typeof value !== 'object' && value !== null && value !== undefined) {
                props[key] = value;
            }
        }
        return props;
    };

    const simpleProps = getSimpleProps();

    return (
        <div className="text-sm">
            <div
                className={`flex items-start gap-2 p-2 rounded transition-colors ${hasChildren
                    ? "cursor-pointer hover:bg-neutral-950"
                    : "hover:bg-neutral-950/50"
                    }`}
                onClick={() => hasChildren && setIsExpanded(!isExpanded)}
            >
                {hasChildren && (
                    <button className="flex-shrink-0 mt-0.5 text-gray-500 hover:text-gray-300 transition-colors">
                        {isExpanded ? (
                            <ChevronDown className="w-3.5 h-3.5" />
                        ) : (
                            <ChevronRight className="w-3.5 h-3.5" />
                        )}
                    </button>
                )}
                <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 flex-wrap">
                        {name && (
                            <span className="text-xs text-gray-500 font-medium">{translatePropertyName(name.replace(/\[\d+\]$/, ''))}:</span>
                        )}
                        <span className={`text-xs font-medium px-1.5 py-0.5 rounded border ${colorClass}`}>
                            {translateNodeType(nodeType)}
                        </span>
                    </div>
                    {Object.keys(simpleProps).length > 0 && (
                        <div className="mt-1 text-xs text-gray-500 flex flex-wrap gap-x-2 gap-y-0.5">
                            {Object.entries(simpleProps).map(([key, value]) => (
                                <span key={key} className="flex items-center gap-1">
                                    <span className="text-gray-600">{translatePropertyName(key)}:</span>
                                    <code className="text-blue-300 bg-blue-500/10 px-1 py-0.5 rounded text-[11px] border border-blue-500/20">
                                        {String(value)}
                                    </code>
                                </span>
                            ))}
                        </div>
                    )}
                </div>
            </div>
            {hasChildren && isExpanded && (
                <div className="ml-4 border-l border-neutral-800 pl-2 mt-0.5 space-y-0.5">
                    {childEntries.map(([key, value]) => {
                        if (Array.isArray(value)) {
                            return value.map((item, index) => (
                                <ASTNodeComponent
                                    key={`${key}-${index}`}
                                    node={item}
                                    depth={depth + 1}
                                    name={`${key}[${index}]`}
                                />
                            ));
                        } else {
                            return (
                                <ASTNodeComponent
                                    key={key}
                                    node={value}
                                    depth={depth + 1}
                                    name={key}
                                />
                            );
                        }
                    })}
                </div>
            )}
        </div>
    );
}

interface ASTViewProps {
    ast: any;
}

export function ASTView({ ast }: ASTViewProps) {
    if (!ast) {
        return (
            <div className="h-full flex items-center justify-center text-gray-400 p-8">
                <div className="text-center space-y-3">
                    <AlertCircle className="w-10 h-10 mx-auto opacity-40" />
                    <p className="text-sm font-medium text-gray-300">Nenhuma AST para exibir</p>
                    <p className="text-xs text-gray-500">Clique em "Analisar" para ver a árvore sintática</p>
                </div>
            </div>
        );
    }

    return (
        <ScrollArea className="h-full">
            <div className="p-4">
                <div className="mb-3 p-3 bg-neutral-950 rounded border border-neutral-800">
                    <h3 className="text-xs font-semibold text-gray-300 mb-1">
                        Árvore Sintática Abstrata
                    </h3>
                    <p className="text-[11px] text-gray-500">
                        Clique nos nós para expandir/recolher
                    </p>
                </div>
                <div className="border border-neutral-800 rounded p-3 bg-black">
                    <ASTNodeComponent node={ast} />
                </div>
            </div>
        </ScrollArea>
    );
}
