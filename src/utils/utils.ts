import { IDocument } from "../types/types";

export const spreadLine = (doc: IDocument) => {
    let line = "";
    if (doc) {
      if (doc.parseTime) line += " " + doc.parseTime;
      if (doc.name) line += " " + doc.name;
      if (doc.organization) line += " " + doc.organization;
      if (doc.docType) line += " " + doc.docType;
      if (doc.number) line += " " + doc.number;
      if (doc.signDate) line += " " + doc.signDate;
    }
    return line;
  };