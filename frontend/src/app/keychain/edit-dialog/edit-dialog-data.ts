import { type } from "@amcharts/amcharts4/core"

export enum RespType {
  DELETE,
  POST,
  CANCEL,
}

export interface EditDialogData {
  type: RespType,
  data?: {[key: string]: any};
}
