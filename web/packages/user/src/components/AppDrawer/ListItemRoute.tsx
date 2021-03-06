import React from 'react';
import { ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import { To, useLocation, useNavigate } from 'react-router-dom';

export interface ListItemRouteProps {
  matchPaths: (string | RegExp)[];
  toPath: To;
  text: React.ReactNode;
  icon: React.ReactNode;
}
export default function ListItemRoute({ matchPaths, toPath, text, icon }: ListItemRouteProps) {
  const { pathname } = useLocation();
  const navigate = useNavigate();
  return (
    <ListItemButton
      selected={matchPaths.some((value) => {
        if (typeof value === 'string') {
          return value === pathname;
        }
        return value.test(pathname);
      })}
      onClick={() => {
        navigate(toPath);
      }}
    >
      <ListItemIcon>{icon}</ListItemIcon>
      <ListItemText primary={text} />
    </ListItemButton>
  );
}
