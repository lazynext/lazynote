import { invoke, invokeAsync } from '../invoke';

export const hello = {
  async helloCommandJson(msg: string[]): Promise<string[]> {
    return (await invoke('hello_command_json', msg))!;
  },
  async helloCommandRaw(data: Uint8Array): Promise<Uint8Array> {
    return (await invoke('hello_command_raw', data, { raw: true }))!;
  },
  async helloCommandVoid() {
    (await invoke('hello_command_void'))!;
  },
  async helloCommandJsonAsync(msg: string[]): Promise<string[]> {
    return (await invokeAsync('hello_command_json_async', msg))!;
  },
  async helloCommandRawAsync(data: Uint8Array): Promise<Uint8Array> {
    return (await invokeAsync('hello_command_raw_async', data, { raw: true }))!;
  },
  async helloCommandVoidAsync() {
    (await invokeAsync('hello_command_void_async'))!;
  },
};
