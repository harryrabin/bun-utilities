/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ExecSubProcess {
  status: string
  output: string
}
export interface ExecOptions {
  cwd?: string
  enviromentVariables?: Record<string, string>
}
export function exec(commandWithArgs: Array<string>, options?: ExecOptions | undefined | null): Promise<ExecSubProcess>
