import { Button, Option, Select } from '@mui/joy';
import { useEffect, useState } from 'react';
import Vditor from 'vditor';

import { api, store } from '@/bindings';
import { os } from '@/config';
import { Link } from '@/router';
import { ColorMode, colorModeList, useColorMode } from '@/utils/theme';

export default function Index() {
  const { colorMode, currentColorMode, setColorMode } = useColorMode();

  const [logs, setLogs] = useState<string[]>([]);
  // const toast = useToast();

  const [vd, setVd] = useState<Vditor>();
  useEffect(() => {
    const vditor = new Vditor('vditor', {
      after: () => {
        vditor.setValue('`Vditor` 最小代码示例');
        setVd(vditor);
      },
      cdn: '/libs/vditor',
      theme: 'classic',
    });
    // Clear the effect
    return () => {
      vd?.destroy();
      setVd(undefined);
    };
  }, []);

  let currentLogs = logs;
  const appendLog = (log: string) => {
    console.log(log);
    const newLogs = [...currentLogs, log];
    currentLogs = newLogs;
    setLogs(newLogs);
  };
  const logTime = (label: string, start: number) => {
    appendLog(`${label} ${(performance.now() - start).toFixed(2)}ms`);
  };

  async function helloCommandJson() {
    const msg = [];
    for (let i = 0; i < 1000000; i++) {
      msg.push(`test ts${i}`);
    }
    const start = performance.now();
    const data = await api.hello.helloCommandJson(msg);
    logTime('hello_command_json', start);

    console.log(data);
  }

  async function helloCommandRaw() {
    const msg = [];
    for (let i = 0; i < 1000000; i++) {
      msg.push(i);
    }
    const data = new Uint8Array(msg);

    const start = performance.now();
    const r = await api.hello.helloCommandRaw(data);
    logTime('hello_command_raw', start);

    console.log(r);
  }

  async function helloCommandVoid() {
    await api.hello.helloCommandVoid();
    appendLog('hello_command_void finish');
  }

  async function helloCommandJsonAsync() {
    const msg = [];
    for (let i = 0; i < 1000000; i++) {
      msg.push(`test ts${i}`);
    }
    const start = performance.now();
    const data = await api.hello.helloCommandJsonAsync(msg);
    logTime('hello_command_json_async', start);
    console.log(data);
  }

  async function helloCommandRawAsync() {
    const msg = [];
    for (let i = 0; i < 1000000; i++) {
      msg.push(i);
    }
    const data = new Uint8Array(msg);
    const start = performance.now();
    const r = await api.hello.helloCommandRawAsync(data);
    logTime('hello_command_raw_async', start);
    console.log(r);
  }

  async function helloCommandVoidAsync() {
    await api.hello.helloCommandVoidAsync();
    appendLog('hello_command_void_async finish');
  }

  async function testStore() {
    const filePath = await store.settings.filePath();
    const test1 = await store.settings.get('test1');

    appendLog(`filePath: ${filePath}\ntest1: ${test1}`);
    await store.settings.set('test1', 'test111_____123');

    await store.settings.save();

    const writeTest1 = await store.settings.get('test1');
    appendLog(`writeTest1: ${writeTest1}`);

    await store.settings.delete('test1');
  }

  async function testOS() {
    appendLog(JSON.stringify(os));
  }

  return (
    <div style={{ backgroundColor: 'transparent', margin: '10px' }}>
      <div>
        <Link to="/about">About Page</Link>
        <div>
          {/* <Separator className="my-4" /> */}
          <div>
            <Button variant="outlined" onClick={() => helloCommandJson()}>
              HelloCommandJson
            </Button>
            <Button variant="outlined" onClick={() => helloCommandRaw()}>
              HelloCommandRaw
            </Button>
            <Button variant="outlined" onClick={() => helloCommandVoid()}>
              HelloCommandVoid
            </Button>
          </div>
          <div>
            <Button variant="outlined" onClick={() => helloCommandJsonAsync()}>
              HelloCommandJsonAsync
            </Button>
            <Button variant="outlined" onClick={() => helloCommandRawAsync()}>
              HelloCommandRawAsync
            </Button>
            <Button variant="outlined" onClick={() => helloCommandVoidAsync()}>
              HelloCommandVoidAsync
            </Button>
          </div>
          <div>
            <Button variant="outlined" onClick={() => testOS()}>
              TestOS
            </Button>
            <Button variant="outlined" onClick={() => testStore()}>
              TestStore
            </Button>
          </div>
          <div>
            {/* <Button onClick={() => toast({ title: 'TestToast', description: 'desc' })}>
              Toast
            </Button> */}
          </div>
          {/* <Separator className="my-4" /> */}
          <div style={{ width: 200 }}>
            <Select
              value={colorMode}
              onChange={(_, newValue) => {
                if (newValue) {
                  setColorMode(newValue as ColorMode);
                }
              }}
            >
              {colorModeList.map((item, index) => (
                <Option key={index} value={item}>
                  {item}
                </Option>
              ))}
            </Select>
            <div>current: {currentColorMode}</div>
            <div>theme: {colorMode}</div>
          </div>
          <div>
            {logs.map((item, index) => (
              <div key={index}>{item}</div>
            ))}
          </div>
          <div id="vditor" className="vditor" />
        </div>
      </div>
    </div>
  );
}
