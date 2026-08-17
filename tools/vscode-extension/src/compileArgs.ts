/**
 * Pure helpers for VS Code → CLI wiring (testable without VS Code API).
 */

export interface CompileOptions {
  compilerPath: string;
  filePath: string;
  target: string;
  framework?: string;
}

/** Build `velin compile ...` argv / shell command from settings. */
export function buildCompileArgs(opts: CompileOptions): string[] {
  const args = [
    'compile',
    '-i',
    opts.filePath,
    '--target',
    opts.target || 'rust',
  ];
  if (opts.framework && opts.framework.trim().length > 0) {
    args.push('--framework', opts.framework.trim());
  }
  return args;
}

export function buildCompileCommand(opts: CompileOptions): string {
  const args = buildCompileArgs(opts);
  const quoted = args.map((a) => (a.includes(' ') ? `"${a}"` : a));
  return `${opts.compilerPath} ${quoted.join(' ')}`;
}

/** Resolve target: prefer compiler.target, fallback velin.target */
export function resolveTarget(compilerTarget?: string, legacyTarget?: string): string {
  return compilerTarget || legacyTarget || 'rust';
}
