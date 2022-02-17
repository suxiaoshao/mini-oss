import React from 'react';
import { ListItemButton, ListItemIcon, ListItemText } from '@mui/material';
import { To, useLocation, useNavigate } from 'react-router-dom';

export interface ListItemRouteProps {
  matchPaths: string[];
  toPath: To;
  text: React.ReactNode;
  icon: React.ReactNode;
}
export default function ListItemRoute({ matchPaths, toPath, text, icon }: ListItemRouteProps) {
  const { pathname } = useLocation();
  const navigate = useNavigate();
  return (
    <ListItemButton
      selected={matchPaths.includes(pathname)}
      onClick={() => {
        navigate(toPath);
      }}
    >
      <ListItemIcon>{icon}</ListItemIcon>
      <ListItemText primary={text} />
    </ListItemButton>
  );
}
