# Spravochnik

> TODO обновить readme, примеры устарели:
> - пути к файлам формируются по новому принципу
> - теперь программа пытается угадать в каком кейсе было имя файла и сделать новое в таком же (пока что работает только для имени файлов, возможно в будущем замена нейминга тоже перейдет на этот алгоритм угадывания вместо алгоритма поэтапной замены)

С помощью этого CLI инструмента вы можете нарушить все заветы реакта и внаглую копипастить код, но заменив весь нейминг (все имена файлов, функции, переменные) на свой.

## Установка
Если у вас уже есть скомпилированный под вашу платформу и установленный файл программы, можете перейти к секции использование. Если нет, и вы хотите скомпилировать его из исходников, вам нужно установить Rust.

После этого перейдите в директорию проекта и выполните команду

```bash
cargo install --path .
```

## Использование

### Команда --help

В этом CLI всегда можно посмотреть подсказки о том какие аргументы куда писать и что они значат. Можно использовать `--help` в корне команды, а так же в ее подкомандах, пользуйтесь.

### Особенности

Алгоритм учитывает что один и тот же нейминг может употребляться в единственном и множественном числах.

Поэтому, если ваш нейминг так же подразумевает такое использование - это можно указать. Тут стоит отметить, что имеет значение использует ли проект, взятый за основу, 2 варианта нейминга, т.к. всё пляшет именно от базового проекта.

Программа определит те места в нем, где есть мн. ч. и подставит туда указанное вами мн. ч. В противном случае подставится все только в ед. ч. (либо, если  вы указывали только один вариант нейминга (например в мн.ч, без ед.ч), соответсвенно он и будет)

Вариант использования если нужен нейминг только в ед.ч.:

```bash
spravochnik default пусть_к_проекту новый_нейминг
```

```bash
spravochnik default ~/Projects/kinoplan/src/js/app/ ribbon_acceptance
```
Тоже самое, но если хотите 2 варианта нейминга:
```bash
spravochnik default ~/Projects/kinoplan/src/js/app/ legal_entity legal_entities
```

### Продвинутый режим
Выше был показан пример использование команды `default`, но есть и другой вариант - команда `clone`. Разница в том, что `default` берет за основу проект по умолчанию, который задается в исходниках в константах (если собираете из исходников, можно открыть их и изменить на то что нужно).

```bash
spravochnik clone пусть_к_проекту основа новый_нейминг [основа_мн_ч] [новый_нейминг_мн_ч]
```
`[необязательное_поле]`

Обратите внимание, что даже если вам не нужно использовать мн.ч. для нового проекта, но у основы 2 варианта нейминга - их оба все равно нужно указать:

```bash
spravochnik clone ~/Projects/kinoplan/src/js/app/ position ribbon_acceptance positions
```

### Режим переименования файлов
Если вам нужно переименовать большое количество файлов, в программе существует команда `rename`. Она *не* задействует продвинутую систему распознавания нейминга в разных регистрах, как команды для копирования разделов. Вместо этого команда заменит буквально ту подстроку, которую укажете. Замена происходит в имени файлов, а так же для всего их содержимого. Имена каталогов останутся без изменений.

Использование:

```bash
spravochnik rename пусть_к_проекту старое_имя новое_имя
```

Например:

```bash
spravochnik rename ~/Projects/kinoplan/src/js/app/ Premiera Prm
```