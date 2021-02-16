import { Pipe, PipeTransform } from '@angular/core';
import { Exchanges } from './entities_pb';

@Pipe({
  name: 'exchangePipe'
})
export class ExchangePipePipe implements PipeTransform {
  private exhcnage_keys = Object.entries(Exchanges).reduce((ret, ent) => {
    const [ key, value ] = ent;
    ret[value] = key;
    return ret;
  });

  transform(value: Exchanges): string {
    return this.exhcnage_keys[value].toString();
  }

}
