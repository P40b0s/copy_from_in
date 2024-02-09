import { IPacket, Task } from "../types/types"

export const mockTasks = async(Task: Task) => {
    return new Promise<Task[]>((resolve) => {
        setTimeout(() => resolve([Task]), 100)
    })
}

export const mockPackets = async() => {
    const pack: IPacket = {
        document: {
            name: "name",
            parseTime: "2024-01-10",
            docType: "Приказ",
            number: "148",
            organization: "FBI",
            signDate: "2024-01-10"
        }
    }
    const err: IPacket = {
        error: "500 - Bad gateway"
    }
    return new Promise<IPacket[]>((resolve) => {
        setTimeout(() => resolve([pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack, err, pack, pack]), 100)
    })
}