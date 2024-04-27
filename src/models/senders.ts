export class Senders
{
    icon?: string;
    id = "";
    medo_addresse?: string;
    notifications_sources_medo_addresses : string[] = [];
    organization = "";
    selected = false;
    update_key = "";
    contact_info : ContactInfoDb[] = [];
    public constructor() { }
    
    public clone() : Senders
    {
        let s = new Senders();
        s.icon = this.icon;
        s.id = this.id;
        s.medo_addresse = this.medo_addresse;
        s.notifications_sources_medo_addresses = this.notifications_sources_medo_addresses;
        s.organization = this.organization;
        s.update_key = this.update_key;
        s.contact_info = this.contact_info;
        s.clone = this.clone;
        s.selected = this.selected;
        return s;
    }
}

export class SendersDb
{
    constructor(s: Senders)
    {
        this.icon = s.icon;
        this.id = s.id;
        this.medo_addresse = s.medo_addresse;
        this.notifications_sources_medo_addresses = s.notifications_sources_medo_addresses
        this.organization = s.organization;
        this.update_key = s.update_key;
        this.contact_info = s.contact_info;
    }
    icon?: string;
    id = "";
    medo_addresse: string | undefined;
    notifications_sources_medo_addresses : string[] = [];
    organization = "";
    update_key = "";
    contact_info: ContactInfoDb[] = [];
}

export class ContactInfoDb
{
    id?: string;
    organization?: string;
    person?: string;
    post?: string;
    contacts: ContactTypeDb[] = [];
    photo?: string;
    note?: string;

}
export class ContactTypeDb
{
    contact_type = "";
    value = "";
}