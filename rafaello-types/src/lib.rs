

pub trait Component {
    /// Тип сообщения
    type Msg;

    /// Обновление внутреннего состояния по сообщению
    fn update(&mut self, msg: Self::Msg);

    /// Рендер на фрейм
    fn view(&self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect);
}
