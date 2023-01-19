import { IRouteableComponent } from '@aurelia/router';
import { invoke } from '@tauri-apps/api';

export class WelcomePage implements IRouteableComponent {
  private message: string;

  private duration = 1;

  private counter = 0;

  async binding() {
    await invoke('return_os', { name: 'Anonymous' })
      .then((response: string) => this.message = response);

    await invoke('return_counter_state').then((counterState: number) => {
      this.counter = counterState;
    })
  }

  playSineWave() {
    invoke("play_sine", { duration: Number(this.duration) });
  }

  updateCounter(value: number) {
    invoke("counter", { countVal: value }).then((counterValue: number) => this.counter = counterValue);
  }
}
