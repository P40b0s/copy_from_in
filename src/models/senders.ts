import { Clone } from "./types";

export class Senders implements Clone<Senders>
{
    icon?: string;
    id = "";
    medo_addresse?: string;
    organization = "";
    contact_info : ContactInfo[] = [];
    public constructor() { }
    public clone(source: Senders) : Senders
    {
        let s = new Senders();
        s.icon = source.icon;
        s.id = source.id;
        s.medo_addresse = source.medo_addresse;
        s.organization = source.organization;
        s.contact_info = source.contact_info;
        s.clone = source.clone;
        return s;
    }
}

export class ContactInfo implements Clone<ContactInfo>
{
    id?: string;
    organization?: string;
    person?: string;
    post?: string;
    photo?: string;
    note?: string;
    contacts : ContactType[] = [];
    public constructor() { }
    
    public clone() : ContactInfo
    {
        let s = new ContactInfo();
        s.id = this.id;
        s.organization = this.organization;
        s.person = this.person;
        s.post = this.post;
        s.photo = this.photo;
        s.note = this.note;
        s.contacts = this.contacts;
        s.clone = this.clone;
        return s;
    }
}

export class ContactType implements Clone<ContactType>
{
    contact_type = "";
    value = "";
    public constructor() { }
    
    public clone() : ContactType
    {
        let s = new ContactType();
        s.contact_type = this.contact_type;
        s.value = this.value;
        s.clone = this.clone;
        return s;
    }
}