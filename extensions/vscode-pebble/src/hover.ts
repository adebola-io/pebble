import * as vscode from "vscode";

export default function registerHover() {
   return vscode.languages.registerHoverProvider("peb", {
      provideHover(document, position, token) {
         console.log(arguments);
         return Promise.resolve(new vscode.Hover("Hello"));
      },
   });
}
