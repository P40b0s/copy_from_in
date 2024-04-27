
export class Requisites
{
    documentGuid?: string;
    actType?: string;
    documentNumber?: string;
    signDate?: string;
    pages?: number;
    annotation?: string;
    mj?: MinistryOfJustice;
}

export class PacketInfo
{
    headerGuid? : string;
    packetDirectory?: string;
    packetType?: string;
    ///Время создания локальной директории
    ///(фактически когда пакет пришел к нам)
    ///зависит от времени на сервере, тому что берет локальное время создания
    deliveryTime? : string;
    wrongEncoding = false;
    error?: string;
    files?: string[];
    requisites?: Requisites;
    senderInfo?: SenderInfo;
    defaultPdf?: string;
    pdfHash?: string;
    acknowledgment?: Ack;
    visible = true;
    updateKey: Date| string = "";
    traceMessage?: string;

}

export class SenderInfo
{
    organization?: string;
    person?: string;
    department?: string;
    post?: string;
    addressee?: string;
    medoAddessee?: string;
    sourceGuid?: string;
    executor?: Executor;
}


export class Executor
{
    organization?: string;
    person?: string;
    post?: string;
    contactInfo?: string;
}

export class MinistryOfJustice
{
    number?: string;
    date?: string;
}

export class Ack
{
    comment?: string;
    accepted = false;
    time?: string;
}