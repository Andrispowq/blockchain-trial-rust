
pub trait DtoConvertible<TargetType> {
    fn to_dto(&self) -> TargetType;
}