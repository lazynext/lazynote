import { MoreVert } from '@mui/icons-material';
import {
  Button,
  Card,
  Dropdown,
  IconButton,
  List,
  ListDivider,
  ListItem,
  ListItemButton,
  Menu,
  MenuButton,
  MenuItem,
  Sheet,
  Stack,
  Typography,
} from '@mui/joy';
import { Fragment } from 'react/jsx-runtime';

import { Logo } from '@/assets/imgs';
import { LanguageSelect, ThemeSelect } from '@/components';
import { appConfig } from '@/config';
import { win } from '@/initialize/initWindow';
import { HistoryWorkspace, useHistoryStore } from '@/models';

import css from './index.module.scss';

const ListItemAuto = (props: { children?: string | React.ReactNode; onClick?: () => void }) => {
  return props.onClick ? (
    <ListItemButton onClick={props.onClick}>{props.children}</ListItemButton>
  ) : (
    <ListItem>{props.children}</ListItem>
  );
};

const ListItemHandle = (props: {
  children?: string | React.ReactNode;
  title?: string;
  desc?: string;
  handle?: React.ReactNode;
  onClick?: () => void;
}) => {
  return (
    <ListItemAuto onClick={props.onClick}>
      <Stack
        width="100%"
        direction="row"
        spacing={2}
        sx={{
          justifyContent: 'space-between',
          alignItems: 'center',
        }}
      >
        <Stack
          direction="row"
          spacing={1}
          sx={{
            justifyContent: 'center',
            alignItems: 'flex-start',
          }}
        >
          {props.children ? (
            props.children
          ) : (
            <>
              <Stack
                direction="column"
                spacing={0}
                sx={{
                  justifyContent: 'center',
                  alignItems: 'flex-start',
                }}
              >
                <Typography level="title-md" textAlign="center">
                  {props.title}
                </Typography>
                <Typography level="body-xs" sx={{ opacity: '90%' }} textAlign="center">
                  {props.desc}
                </Typography>
              </Stack>
            </>
          )}
        </Stack>
        <div>{props.handle}</div>
      </Stack>
    </ListItemAuto>
  );
};

const HistoryItemMenu = (props: { item: HistoryWorkspace }) => {
  const { item } = props;
  return (
    <Dropdown>
      <MenuButton
        slots={{ root: IconButton }}
        slotProps={{
          root: {
            variant: 'plain',
            color: 'neutral',
            onClick: (e) => {
              e.stopPropagation();
              e.preventDefault();
            },
          },
        }}
      >
        <MoreVert />
      </MenuButton>
      <Menu placement="bottom-end">
        <MenuItem
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            openHistoryExplorerClick(item);
          }}
        >
          在资源管理器中打开
        </MenuItem>
        <MenuItem
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            renameHistoryClick(item);
          }}
        >
          重命名工作空间
        </MenuItem>
        <MenuItem
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            removeHistoryClick(item);
          }}
        >
          从列表中移除
        </MenuItem>
      </Menu>
    </Dropdown>
  );
};

const createWorkspaceClick = () => {
  console.log('todo create workspace click');
};

const openWorkspaceClick = () => {
  console.log('todo open workspace click');
};

const openWebdavWorkspaceClick = () => {
  console.log('todo webdav open workspace click');
};

const openHistoryWorkspaceClick = (workspace: HistoryWorkspace) => {
  console.log('todo open history workspace click: ', workspace);
};

const openHistoryExplorerClick = (workspace: HistoryWorkspace) => {
  console.log('todo 在资源管理器中打开 click: ', workspace);
};

const renameHistoryClick = (workspace: HistoryWorkspace) => {
  console.log('todo 重命名工作空间 click: ', workspace);
};

const removeHistoryClick = (workspace: HistoryWorkspace) => {
  console.log('todo 从列表中移除 click: ', workspace);
};

export default function Workspace() {
  const { workspaceHistoryList } = useHistoryStore();
  return (
    <div
      className={css.workspaceRoot}
      onMouseDown={(e) => {
        if (e.buttons === 1) {
          if (e.detail === 2) {
            win.toggleMaximize();
          } else {
            win.startDragging();
          }
        }
      }}
    >
      <Stack
        direction="column"
        spacing={0}
        sx={{
          justifyContent: 'center',
          alignItems: 'center',
          minHeight: '100%',
        }}
      >
        <div className={css.workspaceTop}>
          <Stack
            direction="row"
            spacing={0}
            useFlexGap
            sx={{
              justifyContent: 'center',
              flexWrap: 'wrap',
            }}
          >
            <div className={css.workspaceLogo}>
              <Card
                variant="plain"
                sx={{ height: '270px', width: '100%', backgroundColor: 'transparent' }}
              >
                <Logo size={200} />
                <div>
                  <Typography level="title-lg" textAlign="center">
                    LazyNote
                  </Typography>
                  <Typography level="body-xs" textAlign="center">
                    版本 {appConfig.version}
                  </Typography>
                </div>
              </Card>
            </div>
            <div className={css.workspaceHandle} onMouseDown={(e) => e.stopPropagation()}>
              <List size="md" variant="plain">
                <ListItemHandle
                  handle={
                    <Button color="primary" sx={{ width: 80 }} onClick={createWorkspaceClick}>
                      创建
                    </Button>
                  }
                  title="新建工作空间"
                  desc="创建一个新的工作空间"
                />
                <ListDivider inset="gutter" />
                <ListItemHandle
                  handle={
                    <Button color="neutral" sx={{ width: 80 }} onClick={openWorkspaceClick}>
                      打开
                    </Button>
                  }
                  title="打开工作空间"
                  desc="打开文件夹作为工作空间"
                />
                <ListDivider inset="gutter" />
                <ListItemHandle
                  handle={
                    <Button color="neutral" sx={{ width: 80 }} onClick={openWebdavWorkspaceClick}>
                      打开
                    </Button>
                  }
                  title="Webdav"
                  desc="从Webdav读取并同步到本地"
                />
                <ListDivider inset="gutter" />
                <ListItemHandle
                  title="色彩模式"
                  desc="色彩模式"
                  handle={<ThemeSelect width={150} />}
                />
                <ListDivider inset="gutter" />
                <ListItemHandle
                  title="切换语言"
                  desc="切换语言"
                  handle={<LanguageSelect width={150} />}
                />
              </List>
            </div>
          </Stack>
        </div>
        <div className={css.workspaceHistory}>
          <Sheet variant="plain" sx={{ width: '100%', p: 2, backgroundColor: 'transparent' }}>
            <Typography
              level="h3"
              id="ios-example-demo"
              sx={{ fontSize: 'xl2', fontWeight: 'xl', mb: 1 }}
            ></Typography>
            <List size="md" variant="plain" onMouseDown={(e) => e.stopPropagation()}>
              {workspaceHistoryList?.map((item, index) => {
                return (
                  <Fragment key={index}>
                    <ListItemHandle
                      onClick={() => openHistoryWorkspaceClick(item)}
                      title={item.workspaceName}
                      desc={item.workspacePath}
                      handle={<HistoryItemMenu item={item} />}
                    />
                    {index < workspaceHistoryList.length - 1 && <ListDivider inset="gutter" />}
                  </Fragment>
                );
              })}
            </List>
          </Sheet>
        </div>
      </Stack>
    </div>
  );
}
