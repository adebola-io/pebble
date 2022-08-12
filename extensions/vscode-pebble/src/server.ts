import { createConnection, ProposedFeatures } from "vscode-languageserver/node";

const conn = createConnection(ProposedFeatures.all);

conn.listen();
