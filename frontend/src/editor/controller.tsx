import { editor as monaco, languages as monacoLang, Uri } from 'monaco-editor';


export class Ctrl {
  /**
   * Create typescript definition model for monaco editor.
   *
   * @param defCode definition code.
  */
  public setTsDefModel(defCode: string): void {
    const uri = 'ts:bot-condition.d.ts';
    if (monaco.getModel(Uri.parse(uri))) {
      return;
    }
    const code = defCode;
    const ts = monacoLang.typescript;
    // validation settings
    ts.javascriptDefaults.setDiagnosticsOptions({
      noSemanticValidation: true,
      noSyntaxValidation: false
    });

    // compiler options
    ts.javascriptDefaults.setCompilerOptions({
      target: ts.ScriptTarget.ES2015,
      allowNonTsExtensions: true
    });

    // Model creation
    ts.javascriptDefaults.addExtraLib(code, uri);
    monaco.createModel(
      code, 'typescript', Uri.parse(uri),
    );
  }
}
