import React, { useEffect, useState } from 'react';
import { Label, TextField, PrimaryButton, Separator, Text, DefaultButton, Callout, Stack, Checkbox } from '@fluentui/react';
import { useBoolean } from '@fluentui/react-hooks';
import Send from './Send';

const backend = process.env.BACKEND_URL ?? "https://api.pipehub.net";

function User() {
  const [user, setUser] = useState({} as UserEntity);
  const [wechat, setWechat] = useState({} as Wechat);
  const [isCalloutVisible, { toggle: toggleIsCalloutVisible }] = useBoolean(false);

  useEffect(() => {
    fetch(`${backend}/wechat`, {
      credentials: 'include'
    })
      .then(res => {
        return res.json();
      }).then((wechat: Wechat) => {
        setWechat(wechat);
      })
  }, []);

  useEffect(() => {
    fetch(`${backend}/user`, {
      credentials: 'include'
    })
      .then(res => {
        if (res.status === 401) {
          window.location.href = res.headers.get("Location") ?? "/";
        } else {
          return res.json();
        }
      }).then((entity: UserEntity) => {
        setUser(entity);
      })
  }, []);

  const update = () => {
    fetch(`${backend}/wechat`, {
      method: "PUT",
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(wechat)
    })
      .then(res => {
        if (res.status < 400) {
          alert("Success");
        }
        return res.text();
      }).then(res => {
        console.log(res);
      });

    fetch(`${backend}/user`, {
      method: "PUT",
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(user)
    })
      .then(res => {
        return res.text();
      }).then(res => {
        console.log(res);
      });
  };
  const resetKey = () => {
    toggleIsCalloutVisible();
    fetch(`${backend}/user/reset_key`, {
      method: "POST",
      credentials: 'include',
    })
      .then(res => {
        if (res.status < 400) {
          alert("Success");
        }
        return res.json();
      }).then((entity: UserEntity) => {
        setUser(entity);
      });
  }
  const onCorpIdChange = (event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>, newVal?: string) => {
    setWechat({
      ...wechat,
      corp_id: newVal || '',
    });
  }
  const onAgentIdChange = (event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>, newVal?: string) => {
    setWechat({
      ...wechat,
      agent_id: parseInt(newVal || '0'),
    });
  }
  const onSecretChange = (event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>, newVal?: string) => {
    setWechat({
      ...wechat,
      secret: newVal || '',
    });
  }
  const onBlockListChange = (event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>, newVal?: string) => {
    setUser({
      ...user,
      block_list: newVal || '',
    });
  }

  const onCaptchaChange = (event: React.FormEvent<HTMLInputElement | HTMLElement> | undefined, newVal?: boolean) => {
    setUser({
      ...user,
      captcha: newVal || false,
    });
  }
  const onBotTokenChange = (event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>, newVal?: string) => {
    setWechat({
      ...wechat,
      telegram_bot_token: newVal || '',
    });
  }

  const onChatIdChange = (event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>, newVal?: string) => {
    setWechat({
      ...wechat,
      telegram_chat_id: newVal || '',
    });
  }

  return (
    <div>
      <Label>GitHub ??????</Label> <TextField readOnly value={user.github_login ?? ''}></TextField>
      <Label>App Key</Label> <TextField readOnly value={user.app_key ?? ''}></TextField>

      <Label>?????? ID</Label> <TextField onChange={onCorpIdChange} value={wechat.corp_id ?? ''}></TextField>
      <Label>Agent ID</Label> <TextField onChange={onAgentIdChange} value={wechat.agent_id ? wechat.agent_id.toString() : ""}></TextField>
      <Label>Secret</Label> <TextField onChange={onSecretChange} value={wechat.secret ?? ''}></TextField>
      <Label>Telegram Bot Token</Label> <TextField onChange={onBotTokenChange} value={wechat.telegram_bot_token ?? ''}></TextField>
      <Label>Telegram Chat Id</Label> <TextField onChange={onChatIdChange} value={wechat.telegram_chat_id ?? ''}></TextField>
      <Label>?????????(??????????????????,???????????????????????????, ??????????????????????????????, ???????????????.)</Label> <TextField onChange={onBlockListChange} value={user.block_list ?? ''}></TextField>
      <Checkbox styles={{ root: { marginTop: '10px' } }} label='???????????????(?????????????????????????????????????????????????????????)(Beta)' onChange={onCaptchaChange} checked={user.captcha ?? false} />
      <PrimaryButton style={{ marginTop: '10px' }} onClick={update}>??????</PrimaryButton>

      <DefaultButton
        style={{ marginLeft: '20px' }}
        onClick={toggleIsCalloutVisible}
        id="resetKey"
        text="?????? AppKey"
      />
      {isCalloutVisible ? (
        <div>
          <Callout
            style={{ width: '300px' }}
            target={`#resetKey`}
            onDismiss={toggleIsCalloutVisible}
            setInitialFocus
          >
            <div style={{ padding: '20px 24px 20px' }}>
              <Label>
                ?????? App Key ???????????????????????? URL ???????????????, ???????????????????????????????????? URL. ???????????????????
              </Label>

              <Stack style={{ marginTop: '20px' }} tokens={{ childrenGap: '8px' }} horizontal horizontalAlign="space-evenly">
                <PrimaryButton
                  onClick={resetKey}
                  text="??????"
                />
                <DefaultButton
                  onClick={toggleIsCalloutVisible}
                  text="??????"
                />
              </Stack>
            </div>
          </Callout>
        </div>
      ) : null}

      <Separator />
      <Text variant='xLarge'>?????? Callback URL ???: {user.callback_url}</Text>
      <Separator />
      <Send user={user} />
    </div>
  );
}

export interface UserEntity {
  github_login: string,
  github_id: number,
  app_key: string,
  callback_url: string,
  block_list: string,
  captcha: boolean,
}

export interface Wechat {
  corp_id: string,
  agent_id: number,
  secret: string,
  telegram_bot_token: string,
  telegram_chat_id: string,
}

export default User;
