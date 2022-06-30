import init from '@rust-rpg/rust-rpg-designer';
import { provideFluentDesignSystem, allComponents } from '@fluentui/web-components';
import './styles.scss';

provideFluentDesignSystem().register(allComponents);

init().catch(err => console.error(err));
