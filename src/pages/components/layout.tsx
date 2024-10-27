import { CSSProperties, useState } from 'react';

import { useInited, useTitle } from '@/hooks';
import { getLastHistoryWorkspace, loadWorkspace } from '@/interface';
import Workspace from '@/pages/workspace';
import { Outlet } from '@/router';

const Title = (props: { style?: CSSProperties }) => {
  const { style } = props;
  return <div style={style}></div>;
};

export default function Layout() {
  const [inited, setInited] = useState(false);
  const [showWorkspacePage, setShowWorkspacePage] = useState(false);

  useInited(async () => {
    window.initializeSuccess = true;
    const lastHistoryWorkspace = getLastHistoryWorkspace();
    if (lastHistoryWorkspace) {
      await loadWorkspace(lastHistoryWorkspace.workspacePath);
      setInited(true);
      setShowWorkspacePage(false);
    } else {
      setInited(true);
      setShowWorkspacePage(true);
    }
  });
  const { titleHeight, contentHeight } = useTitle();
  return (
    <>
      <Title style={{ height: titleHeight }} />
      <div style={{ height: contentHeight, overflow: 'auto' }}>
        {inited ? showWorkspacePage ? <Workspace /> : <Outlet /> : undefined}
      </div>
    </>
  );
}
