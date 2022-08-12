import * as vscode from "vscode";
import { pebbleLanguageClient } from "./client";
import registerHover from "./hover";
import registerTokenProvider from "./tokenProvider";

export function activate(context: vscode.ExtensionContext) {
   pebbleLanguageClient.activate(context);
   context.subscriptions.push(registerHover());
   registerTokenProvider();
}

export function deactivate(): Thenable<void> | undefined {
   return pebbleLanguageClient.deactivate();
}
