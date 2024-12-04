
export interface Requisites
{
    documentGuid?: string;
    actType?: string;
    documentNumber?: string;
    signDate?: string;
    pages?: number;
    annotation?: string;
    mj?: MinistryOfJustice;
}

export interface IPacketInfo
{
    headerGuid? : string;
    packetDirectory?: string;
    packetType?: string;
    ///Время создания локальной директории
    ///(фактически когда пакет пришел к нам)
    ///зависит от времени на сервере, тому что берет локальное время создания
    deliveryTime? : string;
    error?: string;
    files?: string[];
    requisites?: Requisites;
    //senderInfo?: SenderInfo;
    senderId?: string,
    defaultPdf?: string;
    pdfHash?: string;
    acknowledgment?: Ack;
    visible: boolean;
    traceMessage?: string;
}

// export interface SenderInfo
// {
//     organization?: string;
//     person?: string;
//     department?: string;
//     post?: string;
//     addressee?: string;
//     medoAddessee?: string;
//     sourceGuid?: string;
//     executor?: Executor;
// }


// export interface Executor
// {
//     organization?: string;
//     person?: string;
//     post?: string;
//     contactInfo?: string;
// }

export interface MinistryOfJustice
{
    number?: string;
    date?: string;
}

export interface Ack
{
    comment?: string;
    accepted: boolean;
    time?: string;
}