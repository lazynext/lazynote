import { Link as JoyLink, LinkProps as JoyLinkProps } from '@mui/joy';

import { navigate } from '@/router';

type LinkProps = JoyLinkProps & {
  to: string;
};

export const Link = (props: LinkProps) => {
  return (
    <JoyLink onClick={() => navigate(props.to)} {...props}>
      {props.children}
    </JoyLink>
  );
};
